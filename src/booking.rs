use std::str::FromStr;

use restson::{RestPath, Error as RestsonError};
use num_traits::{Float,Unsigned};
use serde_derive::{Serialize, Deserialize};

use crate::Error;
use crate::product::{Product,Service};
use crate::account::Account;

/// Enum describing the status of a booking
/// 
/// As defined by the [specification](https://transdirectapiv4.docs.apiary.io/reference/bookings-/-simple-quotes/single-booking)
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
pub enum BookingStatus {
    #[default]
    New,
    PendingPayment,
    Paid,
    RequestSent,
    Reviewed,
    Confirmed,
    Cancelled,
    PendingReview,
    RequestFailed,
    BookedManually,
}

impl FromStr for BookingStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "new"             => Ok(Self::New),
            "pending_payment" => Ok(Self::PendingPayment),
            "paid"            => Ok(Self::Paid),
            "request_sent"    => Ok(Self::RequestSent),
            "reviewed"        => Ok(Self::Reviewed),
            "confirmed"       => Ok(Self::Confirmed),
            "cancelled"       => Ok(Self::Cancelled),
            "pending_review"  => Ok(Self::PendingReview),
            "request_failed"  => Ok(Self::RequestFailed),
            "booked_manually" => Ok(Self::BookedManually),
            _ => Err(Self::Err::UnknownStatus),
        }
    }
}

/// Represents a single booking request (quote or order)
/// 
/// 
#[derive(Debug, Serialize, Default)]
pub struct BookingRequest<'a, T>
where T: Unsigned {
    pub declared_value: f64,
    pub referrer: String,
    pub requesting_site: String,
    pub tailgate_pickup: bool,
    pub tailgate_delivery: bool,
    pub items: Vec<Product<T>>,
    pub sender: Option<&'a Account>,
    pub receiver: Option<&'a Account>,
}

impl<'a, T> BookingRequest<'a, T>
where T: Unsigned + Default {
    /// Creates an empty `BookingRequest`
    /// 
    /// Each element will be either empty, 0, or false.
    /// This provides sensible and convenient defaults for `tailgate_pickup`,
    /// declared_value, etc.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use transdirect::{BookingRequest, Product};
    ///
    /// let products = vec![Product::new()];
    /// let breq = BookingRequest::new()
    ///     .declared_value(55.0)
    ///     .items(&products);
    ///  // .sender(Account { ... })
    ///  // .receiver(Account { ... });
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T> RestPath<()> for BookingRequest<'_, T>
where T: Unsigned {
    fn get_path(_: ()) -> Result<String, RestsonError> { Ok("bookings/v4".to_string()) }
}

/// Represents a response due to a booking request from the server
/// 
///
#[derive(Debug, Deserialize)]
pub struct BookingResponse<T, U> where T: Unsigned, U: Float {
    pub id: u32,
    pub status: BookingStatus,
    #[serde(with = "time::serde::rfc3339")]
    pub booked_at: time::OffsetDateTime,
    pub booked_by: String, // Expected to be "sender"
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: time::OffsetDateTime,
    pub declared_value: U,
    pub insured_value: U,
    pub description: Option<String>,
    pub items: Vec<Product<T>>,
    pub label: String,
    // notifications: 
    pub quotes: Vec<Service<U>>,
    pub sender: Account,
    pub receiver: Account,
    pub pickup_window: Vec<String>,
    pub connote: String,
    pub charged_weight: T,
    pub scanned_weight: T,
    pub special_instructions: String,
    pub tailgate_delivery: bool,
}