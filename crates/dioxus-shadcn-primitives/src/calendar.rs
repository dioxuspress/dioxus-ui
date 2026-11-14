//! Pure date arithmetic and month-grid state for the styled `Calendar` and
//! `DatePicker` components: validation, keyboard moves, and range marking.

/// A day in the proleptic Gregorian calendar, ordered chronologically.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CalendarDate {
  /// Calendar year; may be zero or negative.
  pub year: i32,
  /// Month of the year, 1 (January) to 12 (December).
  pub month: u8,
  /// Day of the month, starting at 1.
  pub day: u8,
}

impl CalendarDate {
  /// Builds a date, or `None` when the month is outside 1..=12 or the day
  /// does not exist in that month (Feb 29 in a common year, say).
  pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
    if !(1..=12).contains(&month) {
      return None;
    }

    if day == 0 || day > days_in_month(year, month) {
      return None;
    }

    Some(Self { year, month, day })
  }

  /// Builds a date without validating it; for constants known to be valid.
  pub const fn unchecked(year: i32, month: u8, day: u8) -> Self {
    Self { year, month, day }
  }

  /// The day of the week this date falls on.
  pub fn weekday(self) -> CalendarWeekday {
    weekday(self)
  }

  /// Steps forward (or backward, when negative) by whole days, crossing month
  /// and year boundaries.
  pub fn add_days(self, days: i32) -> Self {
    add_days(self, days)
  }

  /// Steps by whole months, clamping the day to the target month's last day
  /// (Jan 31 plus one month is Feb 28 or 29).
  pub fn add_months(self, months: i32) -> Self {
    add_months(self, months)
  }

  /// Steps by whole years, clamping Feb 29 to Feb 28 in a common year.
  pub fn add_years(self, years: i32) -> Self {
    add_months(self, years.saturating_mul(12))
  }
}

/// A month of a year, the unit a calendar grid displays.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalendarMonth {
  /// Calendar year.
  pub year: i32,
  /// Month of the year, 1 to 12.
  pub month: u8,
}

impl CalendarMonth {
  /// Builds a month, or `None` when `month` is outside 1..=12.
  pub fn new(year: i32, month: u8) -> Option<Self> {
    if (1..=12).contains(&month) { Some(Self { year, month }) } else { None }
  }

  /// Builds a month without validating it; for constants known to be valid.
  pub const fn unchecked(year: i32, month: u8) -> Self {
    Self { year, month }
  }

  /// The first day of the month.
  pub fn first_day(self) -> CalendarDate {
    CalendarDate::unchecked(self.year, self.month, 1)
  }

  /// Steps by whole months, rolling over into adjacent years.
  pub fn add_months(self, months: i32) -> Self {
    let date = self.first_day().add_months(months);

    Self { year: date.year, month: date.month }
  }
}

/// A day of the week, used to pick the first column of a calendar grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarWeekday {
  /// Sunday.
  Sunday,
  /// Monday.
  Monday,
  /// Tuesday.
  Tuesday,
  /// Wednesday.
  Wednesday,
  /// Thursday.
  Thursday,
  /// Friday.
  Friday,
  /// Saturday.
  Saturday,
}

impl CalendarWeekday {
  /// Zero-based index counting from Sunday (0) to Saturday (6).
  pub const fn number_from_sunday(self) -> u8 {
    match self {
      Self::Sunday => 0,
      Self::Monday => 1,
      Self::Tuesday => 2,
      Self::Wednesday => 3,
      Self::Thursday => 4,
      Self::Friday => 5,
      Self::Saturday => 6,
    }
  }
}

/// Where a day sits relative to a selected date range, for range styling.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CalendarRangeState {
  /// Not in the range.
  #[default]
  Outside,
  /// The range starts and ends on this day.
  Single,
  /// First day of a multi-day range.
  Start,
  /// Strictly between the range's first and last day.
  Middle,
  /// Last day of a multi-day range.
  End,
}

/// A keyboard navigation step through the calendar grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarKeyMove {
  /// One day back (Left arrow).
  PreviousDay,
  /// One day forward (Right arrow).
  NextDay,
  /// Seven days back (Up arrow).
  PreviousWeek,
  /// Seven days forward (Down arrow).
  NextWeek,
  /// One month back, with the day clamped (Page Up).
  PreviousMonth,
  /// One month forward, with the day clamped (Page Down).
  NextMonth,
  /// One year back, with the day clamped (Shift+Page Up).
  PreviousYear,
  /// One year forward, with the day clamped (Shift+Page Down).
  NextYear,
  /// First day of the current week (Home).
  StartOfWeek,
  /// Last day of the current week (End).
  EndOfWeek,
}

/// One cell of a month grid, with the state the styled `Calendar` renders.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalendarDay {
  /// The date this cell shows.
  pub date: CalendarDate,
  /// Whether the date belongs to the previous or next month (padding cells).
  pub outside_month: bool,
  /// Whether the date is the `today` passed to the grid.
  pub today: bool,
  /// Whether the date is the selected date or falls inside the selected range.
  pub selected: bool,
  /// Whether the date is in the disabled list.
  pub disabled: bool,
  /// Position relative to the selected range.
  pub range_state: CalendarRangeState,
}

/// A six-week grid of days covering a month, padded with neighboring days.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarMonthGrid {
  /// The month the grid shows.
  pub month: CalendarMonth,
  /// Six rows of seven days each, starting on the requested first weekday.
  pub weeks: Vec<Vec<CalendarDay>>,
}

/// Whether `year` is a Gregorian leap year.
pub fn is_leap_year(year: i32) -> bool {
  (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Number of days in `month` of `year`, or 0 when the month is outside 1..=12.
pub fn days_in_month(year: i32, month: u8) -> u8 {
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    4 | 6 | 9 | 11 => 30,
    2 if is_leap_year(year) => 29,
    2 => 28,
    _ => 0,
  }
}

/// The date a keyboard step lands on; `first_weekday` decides where
/// `StartOfWeek` and `EndOfWeek` land.
pub fn calendar_move_date(
  date: CalendarDate,
  key_move: CalendarKeyMove,
  first_weekday: CalendarWeekday,
) -> CalendarDate {
  match key_move {
    CalendarKeyMove::PreviousDay => date.add_days(-1),
    CalendarKeyMove::NextDay => date.add_days(1),
    CalendarKeyMove::PreviousWeek => date.add_days(-7),
    CalendarKeyMove::NextWeek => date.add_days(7),
    CalendarKeyMove::PreviousMonth => date.add_months(-1),
    CalendarKeyMove::NextMonth => date.add_months(1),
    CalendarKeyMove::PreviousYear => date.add_years(-1),
    CalendarKeyMove::NextYear => date.add_years(1),
    CalendarKeyMove::StartOfWeek => {
      let offset = weekday_offset(first_weekday, date.weekday()) as i32;
      date.add_days(-offset)
    }
    CalendarKeyMove::EndOfWeek => {
      let offset = weekday_offset(first_weekday, date.weekday()) as i32;
      date.add_days(6 - offset)
    }
  }
}

/// Where `date` sits in the range from `range_start` to `range_end`.
///
/// The ends may come in either order. With only a start, that day is `Single`;
/// with no start, every day is `Outside`.
pub fn calendar_range_state(
  date: CalendarDate,
  range_start: Option<CalendarDate>,
  range_end: Option<CalendarDate>,
) -> CalendarRangeState {
  match (range_start, range_end) {
    (Some(start), Some(end)) if start == end && date == start => CalendarRangeState::Single,
    (Some(start), Some(end)) => {
      let (start, end) = ordered_dates(start, end);

      if date == start {
        CalendarRangeState::Start
      } else if date == end {
        CalendarRangeState::End
      } else if date > start && date < end {
        CalendarRangeState::Middle
      } else {
        CalendarRangeState::Outside
      }
    }
    (Some(start), None) if date == start => CalendarRangeState::Single,
    _ => CalendarRangeState::Outside,
  }
}

/// Builds the six-week grid the styled `Calendar` renders for `month`.
///
/// Rows start on `first_weekday`. `today`, `selected`, and the range ends mark
/// matching cells when `Some`; `None` marks nothing. Dates in `disabled_dates`
/// are flagged but still placed in the grid.
pub fn calendar_month_grid(
  month: CalendarMonth,
  first_weekday: CalendarWeekday,
  today: Option<CalendarDate>,
  selected: Option<CalendarDate>,
  range_start: Option<CalendarDate>,
  range_end: Option<CalendarDate>,
  disabled_dates: &[CalendarDate],
) -> CalendarMonthGrid {
  let first_day = month.first_day();
  let first_offset = weekday_offset(first_weekday, first_day.weekday());
  let mut current = first_day.add_days(-(first_offset as i32));
  let mut weeks = Vec::with_capacity(6);

  for _ in 0..6 {
    let mut week = Vec::with_capacity(7);

    for _ in 0..7 {
      let range_state = calendar_range_state(current, range_start, range_end);
      let selected = selected == Some(current) || range_state != CalendarRangeState::Outside;

      week.push(CalendarDay {
        date: current,
        outside_month: current.month != month.month || current.year != month.year,
        today: today == Some(current),
        selected,
        disabled: disabled_dates.contains(&current),
        range_state,
      });

      current = current.add_days(1);
    }

    weeks.push(week);
  }

  CalendarMonthGrid { month, weeks }
}

fn weekday(date: CalendarDate) -> CalendarWeekday {
  const OFFSETS: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
  let mut year = date.year;

  if date.month < 3 {
    year -= 1;
  }

  let index = (date.month - 1) as usize;
  let weekday =
    (year + year / 4 - year / 100 + year / 400 + OFFSETS[index] + date.day as i32).rem_euclid(7);

  match weekday {
    0 => CalendarWeekday::Sunday,
    1 => CalendarWeekday::Monday,
    2 => CalendarWeekday::Tuesday,
    3 => CalendarWeekday::Wednesday,
    4 => CalendarWeekday::Thursday,
    5 => CalendarWeekday::Friday,
    _ => CalendarWeekday::Saturday,
  }
}

fn add_days(mut date: CalendarDate, days: i32) -> CalendarDate {
  if days >= 0 {
    for _ in 0..days {
      let days_in_current_month = days_in_month(date.year, date.month);

      if date.day < days_in_current_month {
        date.day += 1;
      } else if date.month < 12 {
        date.month += 1;
        date.day = 1;
      } else {
        date.year += 1;
        date.month = 1;
        date.day = 1;
      }
    }
  } else {
    for _ in 0..days.unsigned_abs() {
      if date.day > 1 {
        date.day -= 1;
      } else if date.month > 1 {
        date.month -= 1;
        date.day = days_in_month(date.year, date.month);
      } else {
        date.year -= 1;
        date.month = 12;
        date.day = 31;
      }
    }
  }

  date
}

fn add_months(date: CalendarDate, months: i32) -> CalendarDate {
  let month_index = date.year.saturating_mul(12) + date.month as i32 - 1 + months;
  let year = month_index.div_euclid(12);
  let month = (month_index.rem_euclid(12) + 1) as u8;
  let day = date.day.min(days_in_month(year, month));

  CalendarDate { year, month, day }
}

fn weekday_offset(first_weekday: CalendarWeekday, weekday: CalendarWeekday) -> u8 {
  let first = first_weekday.number_from_sunday();
  let weekday = weekday.number_from_sunday();

  (weekday + 7 - first) % 7
}

fn ordered_dates(first: CalendarDate, second: CalendarDate) -> (CalendarDate, CalendarDate) {
  if first <= second { (first, second) } else { (second, first) }
}

/// Clamps `month` to `1..=12` and `day` to `1..=days_in_month(year, month)`
/// so wheel/dropdown year-month changes always yield a valid `CalendarDate`.
pub fn calendar_clamp_date(year: i32, month: u8, day: u8) -> CalendarDate {
  let month = month.clamp(1, 12);
  let max_day = days_in_month(year, month);
  CalendarDate { year, month, day: day.clamp(1, max_day) }
}

/// A wall-clock time of day (`hour` 0..=23, `minute` 0..=59, `second` 0..=59).
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct CalendarTime {
  /// Hour of the day, 0 to 23.
  pub hour: u8,
  /// Minute of the hour, 0 to 59.
  pub minute: u8,
  /// Second of the minute, 0 to 59.
  pub second: u8,
}

impl CalendarTime {
  /// Builds a time, or `None` when any field is out of 24-hour clock bounds.
  pub fn new(hour: u8, minute: u8, second: u8) -> Option<Self> {
    if hour < 24 && minute < 60 && second < 60 { Some(Self { hour, minute, second }) } else { None }
  }

  /// Builds a time without validating it; for constants known to be valid.
  pub const fn unchecked(hour: u8, minute: u8, second: u8) -> Self {
    Self { hour, minute, second }
  }

  /// Formats as `HH:MM` or `HH:MM:SS`.
  pub fn format(self, include_seconds: bool) -> String {
    format_time(self, include_seconds)
  }
}

/// A combined calendar date and wall-clock time.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CalendarDateTime {
  /// Calendar date.
  pub date: CalendarDate,
  /// Time of day.
  pub time: CalendarTime,
}

impl CalendarDateTime {
  /// Combines a `CalendarDate` and `CalendarTime`.
  pub const fn new(date: CalendarDate, time: CalendarTime) -> Self {
    Self { date, time }
  }

  /// Builds a date-time from raw numeric components, returning `None` if invalid.
  pub fn from_ymd_hms(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
  ) -> Option<Self> {
    Some(Self {
      date: CalendarDate::new(year, month, day)?,
      time: CalendarTime::new(hour, minute, second)?,
    })
  }

  /// Builds a date-time without validating it; for constants known to be valid.
  pub const fn unchecked(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Self {
    Self {
      date: CalendarDate::unchecked(year, month, day),
      time: CalendarTime::unchecked(hour, minute, second),
    }
  }

  /// Formats as `YYYY-MM-DD HH:MM` or `YYYY-MM-DD HH:MM:SS`.
  pub fn format(self, include_seconds: bool) -> String {
    format_date_time(self, include_seconds)
  }
}

/// Formats `time` as `HH:MM` or `HH:MM:SS`.
pub fn format_time(time: CalendarTime, include_seconds: bool) -> String {
  if include_seconds {
    format!("{:02}:{:02}:{:02}", time.hour, time.minute, time.second)
  } else {
    format!("{:02}:{:02}", time.hour, time.minute)
  }
}

/// Parses `HH:MM` or `HH:MM:SS` into a `CalendarTime`.
pub fn parse_time(text: &str) -> Option<CalendarTime> {
  let trimmed = text.trim();
  let mut parts = trimmed.split(':');
  let hour: u8 = parts.next()?.parse().ok()?;
  let minute: u8 = parts.next()?.parse().ok()?;
  let second: u8 = match parts.next() {
    Some(sec) => sec.parse().ok()?,
    None => 0,
  };
  if parts.next().is_some() {
    return None;
  }
  CalendarTime::new(hour, minute, second)
}

/// Formats `value` as `YYYY-MM-DD HH:MM` or `YYYY-MM-DD HH:MM:SS`.
pub fn format_date_time(value: CalendarDateTime, include_seconds: bool) -> String {
  format!(
    "{:04}-{:02}-{:02} {}",
    value.date.year,
    value.date.month,
    value.date.day,
    format_time(value.time, include_seconds),
  )
}

/// Parses `YYYY-MM-DD HH:MM[:SS]` or `YYYY-MM-DDTHH:MM[:SS]` into `CalendarDateTime`.
pub fn parse_date_time(text: &str) -> Option<CalendarDateTime> {
  let trimmed = text.trim();
  let (date_part, time_part) = trimmed.split_once(' ').or_else(|| trimmed.split_once('T'))?;
  let mut d_parts = date_part.split('-');
  let year: i32 = d_parts.next()?.parse().ok()?;
  let month: u8 = d_parts.next()?.parse().ok()?;
  let day: u8 = d_parts.next()?.parse().ok()?;
  if d_parts.next().is_some() {
    return None;
  }
  let date = CalendarDate::new(year, month, day)?;
  let time = parse_time(time_part.trim())?;
  Some(CalendarDateTime::new(date, time))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn validates_calendar_dates() {
    assert_eq!(CalendarDate::new(2024, 2, 29), Some(CalendarDate::unchecked(2024, 2, 29)));
    assert_eq!(CalendarDate::new(2023, 2, 29), None);
    assert_eq!(CalendarDate::new(2024, 13, 1), None);
  }

  #[test]
  fn calculates_leap_year_month_lengths() {
    assert_eq!(days_in_month(2024, 2), 29);
    assert_eq!(days_in_month(2100, 2), 28);
    assert_eq!(days_in_month(2000, 2), 29);
  }

  #[test]
  fn moves_across_month_boundaries() {
    let date = CalendarDate::unchecked(2024, 3, 1);

    assert_eq!(date.add_days(-1), CalendarDate::unchecked(2024, 2, 29));
    assert_eq!(date.add_months(-1), CalendarDate::unchecked(2024, 2, 1));
    assert_eq!(
      CalendarDate::unchecked(2024, 1, 31).add_months(1),
      CalendarDate::unchecked(2024, 2, 29)
    );
  }

  #[test]
  fn builds_six_week_month_grid_with_outside_days() {
    let month = CalendarMonth::unchecked(2024, 6);
    let grid = calendar_month_grid(month, CalendarWeekday::Sunday, None, None, None, None, &[]);

    assert_eq!(grid.weeks.len(), 6);
    assert_eq!(grid.weeks[0].len(), 7);
    assert_eq!(grid.weeks[0][0].date, CalendarDate::unchecked(2024, 5, 26));
    assert!(grid.weeks[0][0].outside_month);
  }

  #[test]
  fn marks_today_selected_disabled_and_range_days() {
    let month = CalendarMonth::unchecked(2024, 6);
    let disabled = [CalendarDate::unchecked(2024, 6, 12)];
    let grid = calendar_month_grid(
      month,
      CalendarWeekday::Monday,
      Some(CalendarDate::unchecked(2024, 6, 10)),
      Some(CalendarDate::unchecked(2024, 6, 11)),
      Some(CalendarDate::unchecked(2024, 6, 12)),
      Some(CalendarDate::unchecked(2024, 6, 14)),
      &disabled,
    );
    let days: Vec<CalendarDay> = grid.weeks.into_iter().flatten().collect();

    let today = days.iter().find(|day| day.date == CalendarDate::unchecked(2024, 6, 10)).copied();
    let selected =
      days.iter().find(|day| day.date == CalendarDate::unchecked(2024, 6, 11)).copied();
    let range_middle =
      days.iter().find(|day| day.date == CalendarDate::unchecked(2024, 6, 13)).copied();

    assert_eq!(today.map(|day| day.today), Some(true));
    assert_eq!(selected.map(|day| day.selected), Some(true));
    assert_eq!(range_middle.map(|day| day.selected), Some(true));
    assert_eq!(range_middle.map(|day| day.range_state), Some(CalendarRangeState::Middle));
    assert_eq!(
      days
        .iter()
        .find(|day| day.date == CalendarDate::unchecked(2024, 6, 12))
        .map(|day| day.disabled),
      Some(true)
    );
  }

  #[test]
  fn moves_dates_with_keyboard_semantics() {
    let date = CalendarDate::unchecked(2024, 6, 5);

    assert_eq!(
      calendar_move_date(date, CalendarKeyMove::NextWeek, CalendarWeekday::Monday),
      CalendarDate::unchecked(2024, 6, 12)
    );
    assert_eq!(
      calendar_move_date(date, CalendarKeyMove::StartOfWeek, CalendarWeekday::Monday),
      CalendarDate::unchecked(2024, 6, 3)
    );
    assert_eq!(
      calendar_move_date(date, CalendarKeyMove::EndOfWeek, CalendarWeekday::Monday),
      CalendarDate::unchecked(2024, 6, 9)
    );
  }

  #[test]
  fn classifies_reversed_ranges() {
    assert_eq!(
      calendar_range_state(
        CalendarDate::unchecked(2024, 6, 13),
        Some(CalendarDate::unchecked(2024, 6, 14)),
        Some(CalendarDate::unchecked(2024, 6, 12)),
      ),
      CalendarRangeState::Middle
    );
  }

  #[test]
  fn clamps_wheel_dates_and_parses_date_times() {
    assert_eq!(calendar_clamp_date(2024, 2, 31), CalendarDate::unchecked(2024, 2, 29));
    assert_eq!(calendar_clamp_date(2025, 2, 31), CalendarDate::unchecked(2025, 2, 28));

    let dt = parse_date_time("2026-10-15 14:30:45");
    assert_eq!(dt, Some(CalendarDateTime::unchecked(2026, 10, 15, 14, 30, 45)));
    assert_eq!(
      parse_date_time("2026-10-15T09:05"),
      Some(CalendarDateTime::unchecked(2026, 10, 15, 9, 5, 0))
    );
    assert_eq!(parse_date_time("2026-02-30 12:00"), None);
    assert_eq!(
      format_date_time(CalendarDateTime::unchecked(2026, 10, 15, 9, 5, 0), false),
      "2026-10-15 09:05"
    );
  }
}
