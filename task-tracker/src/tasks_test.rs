use crate::tasks::Task;
#[cfg(test)]
use chrono::DateTime;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
// #[allow(dead_code)]
trait Trait {
    fn add(&self, input: &str) -> Task;
    fn list(&self) -> Task;
}

#[cfg(test)]
fn add(input: &str) -> Task {
    return Task {
        id: 1,
        description: input.to_string(),
        status: String::from("todo"),
        created_at: DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
        updated_at: DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
    };
}

#[cfg(test)]
fn update(id: i32, input: &str) -> Task {
    let test_update = Task {
        id,
        description: input.to_string(),
        status: String::from("todo"),
        created_at: DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
        updated_at: DateTime::parse_from_str(
            "2025-04-10 14:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
    };
    test_update
}

#[cfg(test)]
fn delete(id: i32) -> bool {
    if id != 1 {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Local, TimeZone, Utc};

    use crate::tasks_test::{self, add};

    use chrono_tz::{Asia, Tz};

    fn get_task_add(id: i32, description: &str) -> Task {
        return Task {
            id,
            description: String::from(description),
            status: String::from("todo"),
            created_at: DateTime::parse_from_str(
                "2025-04-10 10:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
            updated_at: DateTime::parse_from_str(
                "2025-04-10 10:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
        };
    }

    fn get_task_update(id: i32, description: &str) -> Task {
        let mut test_add = get_task_add(id, description);
        test_add.updated_at = DateTime::parse_from_str(
            "2025-04-10 14:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into();
        test_add
    }

    #[test]
    fn test_date_time_with_time_zone() {
        let utc_date_time: DateTime<Utc> = Utc::now();
        let asia_jakarta_date_time: DateTime<Tz> =
            Asia::Jakarta.from_utc_datetime(&utc_date_time.naive_utc());

        println!("{}", utc_date_time);
        println!("{}", asia_jakarta_date_time);

        let local_date_time: DateTime<Local> = Local::now();
        let asia_jakarta_date_time: DateTime<Tz> = Asia::Jakarta
            .from_local_datetime(&local_date_time.naive_local())
            .unwrap();

        println!("{}", local_date_time);
        println!("{}", asia_jakarta_date_time);
    }

    #[test]
    fn test_add_success() {
        let test_task = tasks_test::Task {
            id: 1,
            description: String::from("test buy eggs"),
            status: String::from("todo"),
            created_at: DateTime::parse_from_str(
                "2025-04-10 10:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
            updated_at: DateTime::parse_from_str(
                "2025-04-10 10:10:10.000000 +07:00",
                "%Y-%m-%d %H:%M:%S%.6f %z",
            )
            .unwrap()
            .into(),
        };

        assert_eq!(test_task, add("test buy eggs"));
    }

    use super::*;

    #[test]
    fn test_mock_add_success() {
        let mut mock = MockTrait::new();
        mock.expect_add()
            .with(eq("test buy eggs 2"))
            .times(1)
            .returning(|x| get_task_add(2, x));
        assert_eq!(
            get_task_add(2, "test buy eggs 2"),
            mock.add("test buy eggs 2")
        );
    }

    #[test]
    fn test_update_success() {
        let test_task = get_task_update(3, "test buy 3 eggs");

        assert_eq!(test_task, update(3, "test buy 3 eggs"));
    }

    #[test]
    fn test_delete_success() {
        let test_task = delete(1);
        assert_eq!(test_task, true);
    }

    #[test]
    fn test_delete_fail() {
        let test_task = delete(2);
        assert_eq!(test_task, false);

        let test_task = delete(-1);
        assert_eq!(test_task, false);
    }

    #[test]
    fn test_mock_list_success() {
        let mut mock = MockTrait::new();
        mock.expect_list()
            .times(1)
            .returning(|| (get_task_add(2, "test buy eggs 4")));
        assert_eq!(get_task_add(2, "test buy eggs 4"), mock.list());
    }
}
