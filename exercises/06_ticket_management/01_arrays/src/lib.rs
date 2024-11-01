// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.
#[derive(PartialEq, Debug)]
pub struct WeekTemperatures {
    monday: Option<i32>,
    tuesday: Option<i32>,
    wednesday: Option<i32>,
    thursday: Option<i32>,
    friday: Option<i32>,
    saturday: Option<i32>,
    sunday: Option<i32>,
}

#[derive(PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        WeekTemperatures {
            monday: None,
            tuesday: None,
            wednesday: None,
            thursday: None,
            friday: None,
            saturday: None, 
            sunday: None
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        if day == Weekday::Monday {
            return self.monday
        } else if day == Weekday::Tuesday {
            return self.tuesday
        } else if day == Weekday::Wednesday {
            return self.wednesday
        } else if day == Weekday::Thursday {
            return self.thursday
        } else if day == Weekday::Friday {
            return self.friday
        } else if day == Weekday::Saturday {
            return self.saturday
        }
        return self.sunday
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        if day == Weekday::Monday {
            self.monday = Some(temperature)
        } else if day == Weekday::Tuesday {
            self.tuesday = Some(temperature)
        } else if day == Weekday::Wednesday {
            self.wednesday = Some(temperature)
        } else if day == Weekday::Thursday {
            self.thursday = Some(temperature)
        } else if day == Weekday::Friday {
            self.friday = Some(temperature)
        } else if day == Weekday::Saturday {
            self.saturday = Some(temperature)
        } else {
            self.sunday = Some(temperature)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
