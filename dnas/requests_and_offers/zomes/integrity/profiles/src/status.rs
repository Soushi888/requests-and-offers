use std::{fmt::Display, str::FromStr};

// use chrono::Duration;
use hdi::prelude::Timestamp;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Status {
    #[default]
    Pending,
    Accepted,
    Rejected,
    Suspended(Option<Timestamp>),
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Accepted => write!(f, "accepted"),
            Self::Rejected => write!(f, "rejected"),
            Self::Suspended(Some(time)) => write!(f, "suspended until {}", time),
            Self::Suspended(None) => write!(f, "suspended indefinitely"),
        }
    }
}

#[derive(Debug, Default)]
pub enum StatusParsingError {
    #[default]
    InvalidStatus,
    InvalidTimestamp,
}

impl Display for StatusParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStatus => write!(f, "Invalid status"),
            Self::InvalidTimestamp => write!(f, "Invalid timestamp"),
        }
    }
}

impl FromStr for Status {
    type Err = StatusParsingError;

    fn from_str(status: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = status.split_whitespace().collect();
        match parts.as_slice() {
            ["pending"] => Ok(Self::Pending),
            ["accepted"] => Ok(Self::Accepted),
            ["rejected"] => Ok(Self::Rejected),
            ["suspended"] => Ok(Self::Suspended(None)),
            ["suspended", timestamp] => Ok(Self::Suspended(Some(
                Timestamp::from_str(timestamp).or(Err(StatusParsingError::InvalidTimestamp))?,
            ))),
            _ => Err(StatusParsingError::InvalidStatus),
        }
    }
}

impl From<&str> for Status {
    fn from(status: &str) -> Self {
        Self::from_str(status).unwrap_or_default()
    }
}

impl Status {
    // pub fn get_suspension_time_remaining(&self) -> Option<Duration> {
    //     let now = Timestamp::now();
    //     match self {
    //         Status::Suspended(time) if time.is_some() => Some(
    //             time.unwrap()
    //                 .checked_difference_signed(&now)
    //                 .unwrap_or(Duration::zero()),
    //         ),
    //         _ => None,
    //     }
    // }

    // pub fn suspend(&mut self, time: Option<Duration>) {
    //     let now = Timestamp::now().as_micros();

    //     if time.is_none() {
    //         *self = Status::Suspended(None);
    //         return;
    //     }

    //     let time = time.unwrap().num_microseconds().unwrap_or(0);
    //     match self {
    //         Status::Suspended(timestamp) if timestamp.is_some() => {
    //             *self = Status::Suspended(Some(Timestamp::from_micros(
    //                 timestamp.unwrap().as_micros() + time,
    //             )))
    //         }
    //         _ => *self = Status::Suspended(Some(Timestamp::from_micros(now + time))),
    //     }
    // }

    // pub fn unsuspend_if_time_passed(&mut self) {
    //     if let Some(time) = self.get_suspension_time_remaining() {
    //         println!("Time remaining: {:?}", time);
    //         if time.is_zero() || time < Duration::hours(1) {
    //             *self = Status::Accepted
    //         }
    //     }
    // }

    pub fn unsuspend(&mut self) {
        *self = Status::Accepted
    }
}
