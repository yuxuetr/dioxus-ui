#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CalendarDate {
  pub year: i32,
  pub month: u8,
  pub day: u8,
}

impl CalendarDate {
  pub fn new(year: i32, month: u8, day: u8) -> Option<Self> {
    if !(1..=12).contains(&month) {
      return None;
    }

    if day == 0 || day > days_in_month(year, month) {
      return None;
    }

    Some(Self { year, month, day })
  }

  pub const fn unchecked(year: i32, month: u8, day: u8) -> Self {
    Self { year, month, day }
  }

  pub fn weekday(self) -> CalendarWeekday {
    weekday(self)
  }

  pub fn add_days(self, days: i32) -> Self {
    add_days(self, days)
  }

  pub fn add_months(self, months: i32) -> Self {
    add_months(self, months)
  }

  pub fn add_years(self, years: i32) -> Self {
    add_months(self, years.saturating_mul(12))
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalendarMonth {
  pub year: i32,
  pub month: u8,
}

impl CalendarMonth {
  pub fn new(year: i32, month: u8) -> Option<Self> {
    if (1..=12).contains(&month) {
      Some(Self { year, month })
    } else {
      None
    }
  }

  pub const fn unchecked(year: i32, month: u8) -> Self {
    Self { year, month }
  }

  pub fn first_day(self) -> CalendarDate {
    CalendarDate::unchecked(self.year, self.month, 1)
  }

  pub fn add_months(self, months: i32) -> Self {
    let date = self.first_day().add_months(months);

    Self {
      year: date.year,
      month: date.month,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarWeekday {
  Sunday,
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
}

impl CalendarWeekday {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarRangeState {
  Outside,
  Single,
  Start,
  Middle,
  End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CalendarKeyMove {
  PreviousDay,
  NextDay,
  PreviousWeek,
  NextWeek,
  PreviousMonth,
  NextMonth,
  PreviousYear,
  NextYear,
  StartOfWeek,
  EndOfWeek,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CalendarDay {
  pub date: CalendarDate,
  pub outside_month: bool,
  pub today: bool,
  pub selected: bool,
  pub disabled: bool,
  pub range_state: CalendarRangeState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarMonthGrid {
  pub month: CalendarMonth,
  pub weeks: Vec<Vec<CalendarDay>>,
}

pub fn is_leap_year(year: i32) -> bool {
  (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
    4 | 6 | 9 | 11 => 30,
    2 if is_leap_year(year) => 29,
    2 => 28,
    _ => 0,
  }
}

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
  let weekday = (year + year / 4 - year / 100 + year / 400 + OFFSETS[index] + date.day as i32)
    .rem_euclid(7);

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
  if first <= second {
    (first, second)
  } else {
    (second, first)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn validates_calendar_dates() {
    assert_eq!(
      CalendarDate::new(2024, 2, 29),
      Some(CalendarDate::unchecked(2024, 2, 29))
    );
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
    let grid = calendar_month_grid(
      month,
      CalendarWeekday::Sunday,
      None,
      None,
      None,
      None,
      &[],
    );

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

    let today = days
      .iter()
      .find(|day| day.date == CalendarDate::unchecked(2024, 6, 10))
      .copied();
    let selected = days
      .iter()
      .find(|day| day.date == CalendarDate::unchecked(2024, 6, 11))
      .copied();
    let range_middle = days
      .iter()
      .find(|day| day.date == CalendarDate::unchecked(2024, 6, 13))
      .copied();

    assert_eq!(today.map(|day| day.today), Some(true));
    assert_eq!(selected.map(|day| day.selected), Some(true));
    assert_eq!(range_middle.map(|day| day.selected), Some(true));
    assert_eq!(
      range_middle.map(|day| day.range_state),
      Some(CalendarRangeState::Middle)
    );
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
}
