use std::collections::HashMap;

use restson::{RestPath, Error as RestsonError};
use num_traits::{Float,Unsigned};
use serde_derive::{Serialize, Deserialize};
use serde::{de, ser};

use crate::product::{Product,Service};
use crate::account::Account;

/// Enum describing the status of a booking
/// 
/// As defined by the [specification](https://transdirectapiv4.docs.apiary.io/reference/bookings-/-simple-quotes/single-booking)
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Default)]
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

impl<'de> de::Deserialize<'de> for BookingStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: de::Deserializer<'de>
    {
        let variant = String::deserialize(deserializer)?;
        match variant.as_str() {
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
            _   => Err(de::Error::custom("Unrecognised enum value"))
        }
    }
}

/// Represents a single booking request (quote or order)
/// 
/// 
#[derive(Debug, Serialize, Default)]
pub struct BookingRequest<'a, T, U>
where T: Unsigned + ser::Serialize, U: Float + ser::Serialize {
    pub declared_value: U,
    pub referrer: String,
    pub requesting_site: String,
    pub tailgate_pickup: bool,
    pub tailgate_delivery: bool,
    pub items: Vec<Product<T, U>>, // Products may be in a higher scope
    pub sender: Option<&'a Account>,
    pub receiver: Option<&'a Account>,
}

impl<'a, T, U> BookingRequest<'a, T, U>
where T: Unsigned + ser::Serialize + Default, U: Float + ser::Serialize + Default {
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
    /// # use transdirect::Account;
    /// # let person = Account::default();
    /// 
    /// let products = vec![Product::new()];
    /// let breq = BookingRequest {
    ///     declared_value: 55.0,
    ///     items: products,
    ///     sender: Some(&person),
    ///     receiver: Some(&person),
    ///     ..BookingRequest::default()
    /// };
    /// ```
    pub fn new() -> Self {
        Default::default()
    }
}

impl<T, U> RestPath<()> for BookingRequest<'_, T, U>
where T: Unsigned + ser::Serialize, U: Float + ser::Serialize {
    fn get_path(_: ()) -> Result<String, RestsonError> { Ok("bookings/v4".to_string()) }
}

// I don't know how to implement generically without running into collisions
impl<T, U> RestPath<u32> for BookingResponse<T, U>
where T: Unsigned, U: Float {
    fn get_path(params: u32) -> Result<String, RestsonError> {
        Ok(format!("bookings/v4/{params}"))
    }
}

/// Represents a response due to a booking request from the server
/// 
///
#[derive(Debug, Deserialize)]
pub struct BookingResponse<T, U>
where T: Unsigned, U: Float {
    pub id: u32,
    pub status: BookingStatus,
    #[serde(with = "time::serde::iso8601")]
    pub booked_at: time::OffsetDateTime,
    pub booked_by: String, // Expected to be "sender"
    #[serde(with = "time::serde::iso8601")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: time::OffsetDateTime,
    pub declared_value: U,
    pub insured_value: U,
    pub description: Option<String>,
    pub items: Vec<Product<T, U>>,
    pub label: String,
    pub notifications: HashMap<String, bool>,
    pub quotes: HashMap<String, Service<U>>,
    pub sender: Account,
    pub receiver: Account,
    pub pickup_window: Vec<String>, // Could be a time::OffsetDateTime
    pub connote: Option<String>, // With the mock server, this is null => None
    pub charged_weight: T,
    pub scanned_weight: T,
    pub special_instructions: String,
    pub tailgate_delivery: bool,
}