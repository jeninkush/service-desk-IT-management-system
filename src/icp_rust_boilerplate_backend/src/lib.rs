#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// UserRole Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum UserRole {
    #[default]
    Admin,
    ITSupport,
    User,
}

// Ticket Status Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum TicketStatus {
    #[default]
    Open,
    InProgress,
    Closed,
}

// Ticket Priority Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum TicketPriority {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Ticket {
    id: u64,
    title: String,
    description: String,
    status: TicketStatus,
    priority: TicketPriority,
    created_at: u64,
    created_by: u64,
    assigned_to: Option<u64>, // ITSupport user ID
    history: Vec<TicketHistory>,
    comments: Vec<Comment>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct TicketHistory {
    status: String,
    changed_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Comment {
    user_id: u64,
    content: String,
    commented_at: u64,
}

// Asset Type Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum AssetType {
    #[default]
    Laptop,
    Desktop,
    Monitor,
    Printer,
    Scanner,
    Other,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ITAsset {
    id: u64,
    asset_name: String,
    asset_type: AssetType,
    purchase_date: u64,
    assigned_to: u64,
    approx_value: f64,      // Approximate value of the asset
    depreciation_rate: f64, // Annual depreciation rate as a percentage
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    role: UserRole,
    created_at: u64,
}

impl Storable for Ticket {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Ticket {
    const MAX_SIZE: u32 = 4096;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for ITAsset {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for ITAsset {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static TICKET_STORAGE: RefCell<StableBTreeMap<u64, Ticket, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static IT_ASSET_STORAGE: RefCell<StableBTreeMap<u64, ITAsset, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

// Ticket Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct TicketPayload {
    title: String,
    description: String,
    priority: TicketPriority,
}

// ITAssetPayload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct ITAssetPayload {
    asset_name: String,
    asset_type: AssetType,
    purchase_date: u64,
    assigned_to: u64,
    approx_value: f64,
    depreciation_rate: f64,
}

// UserPayload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    username: String,
    role: UserRole,
}

// calculate_depreciation Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct CalculateDepreciationPayload {
    it_asset_id: u64,
    years: u64,
}

// assign_ticket
#[derive(candid::CandidType, Deserialize, Serialize)]
struct AssignTicketPayload {
    ticket_id: u64,
    assigned_to: u64,
}

// add_ticket_comment
#[derive(candid::CandidType, Deserialize, Serialize)]
struct AddTicketCommentPayload {
    ticket_id: u64,
    user_id: u64,
    content: String,
}

// update_ticket_status Payload
#[derive(candid::CandidType, Deserialize, Serialize)]
struct UpdateTicketStatusPayload {
    id: u64,
    status: TicketStatus,
}

// Meassge
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    UnAuthorized(String),
}

#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, Message> {
    if payload.username.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'username' and 'role' are provided.".to_string(),
        ));
    }

    // Check if the user already exists
    let user_exists = USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, user)| user.username == payload.username)
    });
    if user_exists {
        return Err(Message::Error("User already exists".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let user = User {
        id,
        username: payload.username,
        role: payload.role,
        created_at: current_time(),
    };
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

#[ic_cdk::query]
fn get_users() -> Result<Vec<User>, Message> {
    USER_STORAGE.with(|storage| {
        let users: Vec<User> = storage
            .borrow()
            .iter()
            .map(|(_, user)| user.clone())
            .collect();

        if users.is_empty() {
            Err(Message::NotFound("No users found".to_string()))
        } else {
            Ok(users)
        }
    })
}

#[ic_cdk::query]
fn get_user_by_id(id: u64) -> Result<User, Message> {
    USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, user)| user.id == id)
            .map(|(_, user)| user.clone())
            .ok_or(Message::NotFound("User not found".to_string()))
    })
}

// User authentication
fn authenticate_user(payload: UserPayload) -> Result<User, Message> {
    USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, user)| user.username == payload.username && user.role == payload.role)
            .map(|(_, user)| user.clone())
            .ok_or(Message::UnAuthorized("Invalid credentials".to_string()))
    })
}

// Fuction to create ticket
#[ic_cdk::update]
fn create_ticket(payload: TicketPayload, user_payload: UserPayload) -> Result<Ticket, Message> {
    // Authenticate the user
    let user = authenticate_user(user_payload)?;
    if user.role != UserRole::User && user.role != UserRole::Admin {
        return Err(Message::UnAuthorized(
            "You do not have permission to create a ticket".to_string(),
        ));
    }

    // Ensure 'title' and 'description' are provided
    if payload.title.is_empty() || payload.description.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'title' and 'description' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let ticket = Ticket {
        id,
        title: payload.title,
        description: payload.description,
        status: TicketStatus::Open,
        priority: payload.priority,
        created_at: current_time(),
        created_by: user.id,
        assigned_to: None,
        history: vec![],
        comments: vec![],
    };
    TICKET_STORAGE.with(|storage| storage.borrow_mut().insert(id, ticket.clone()));
    Ok(ticket)
}

// Function to assign a ticket
#[ic_cdk::update]
fn assign_ticket(
    payload: AssignTicketPayload,
    user_payload: UserPayload,
) -> Result<Ticket, Message> {
    // Authenticate the user
    let user = authenticate_user(user_payload)?;
    if user.role != UserRole::ITSupport && user.role != UserRole::Admin {
        return Err(Message::UnAuthorized(
            "You do not have permission to assign a ticket".to_string(),
        ));
    }

    // Validate the assigned user
    let assigned_user_exists = USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, user)| user.id == payload.assigned_to)
    });

    if !assigned_user_exists {
        return Err(Message::InvalidPayload(
            "Assigned user does not exist".to_string(),
        ));
    }

    // Check if the ticket exists
    let ticket_exists = TICKET_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, ticket)| ticket.id == payload.ticket_id)
    });

    if !ticket_exists {
        return Err(Message::NotFound("Ticket not found".to_string()));
    }

    TICKET_STORAGE.with(|storage| {
        let mut storage_ref = storage.borrow_mut();
        if let Some(ticket) = storage_ref.get(&payload.ticket_id).iter_mut().next() {
            ticket.assigned_to = Some(payload.assigned_to);
            storage_ref.insert(payload.ticket_id, ticket.clone());
            Ok(ticket.clone())
        } else {
            Err(Message::NotFound("Ticket not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn update_ticket_status(payload: UpdateTicketStatusPayload) -> Result<Ticket, Message> {
    TICKET_STORAGE.with(|ticket_storage| {
        let mut storage_ref = ticket_storage.borrow_mut();

        // Check if the ticket exists
        if let Some(mut ticket) = storage_ref.get(&payload.id) {
            // Update the status
            ticket.status = payload.status;

            // Update the history
            ticket.history.push(TicketHistory {
                status: format!("{:?}", payload.status),
                changed_at: current_time(),
            });

            // Update the ticket in storage
            storage_ref.insert(payload.id, ticket.clone());

            Ok(ticket.clone())
        } else {
            Err(Message::NotFound("Ticket not found".to_string()))
        }
    })
}

// Function to add_ticket_comment
#[ic_cdk::update]
fn add_ticket_comment(payload: AddTicketCommentPayload) -> Result<Ticket, Message> {
    TICKET_STORAGE.with(|ticket_storage| {
        let mut storage_ref = ticket_storage.borrow_mut();

        // Check if the ticket exists
        if let Some(mut ticket) = storage_ref.get(&payload.ticket_id) {
            // Check if the user exists
            let user_exists = USER_STORAGE.with(|user_storage| {
                user_storage
                    .borrow()
                    .iter()
                    .any(|(_, user)| user.id == payload.user_id)
            });

            if !user_exists {
                return Err(Message::InvalidPayload("User does not exist".to_string()));
            }

            // Add the comment
            ticket.comments.push(Comment {
                user_id: payload.user_id,
                content: payload.content,
                commented_at: current_time(),
            });

            // Update the ticket in storage
            storage_ref.insert(payload.ticket_id, ticket.clone());

            Ok(ticket.clone())
        } else {
            Err(Message::NotFound("Ticket not found".to_string()))
        }
    })
}

// Function to get Tickets
#[ic_cdk::query]
fn get_tickets() -> Result<Vec<Ticket>, Message> {
    TICKET_STORAGE.with(|storage| {
        let tickets: Vec<Ticket> = storage
            .borrow()
            .iter()
            .map(|(_, ticket)| ticket.clone())
            .collect();

        if tickets.is_empty() {
            Err(Message::NotFound("No tickets found".to_string()))
        } else {
            Ok(tickets)
        }
    })
}

#[ic_cdk::query]
fn get_ticket_by_id(id: u64) -> Result<Ticket, Message> {
    TICKET_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, ticket)| ticket.id == id)
            .map(|(_, ticket)| ticket.clone())
            .ok_or(Message::NotFound("Ticket not found".to_string()))
    })
}

// Function to create an asset
#[ic_cdk::update]
fn create_it_asset(payload: ITAssetPayload, user_payload: UserPayload) -> Result<ITAsset, Message> {
    // Authenticate the user
    let user = authenticate_user(user_payload)?;
    if user.role != UserRole::ITSupport && user.role != UserRole::Admin {
        return Err(Message::UnAuthorized(
            "You do not have permission to create an IT asset.".to_string(),
        ));
    }

    // Ensure 'asset_name' and 'asset_type' are provided
    if payload.asset_name.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'asset_name' and 'asset_type' are provided.".to_string(),
        ));
    }

    // Validate the assigned user
    let assigned_user_exists = USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, user)| user.id == payload.assigned_to)
    });

    if !assigned_user_exists {
        return Err(Message::InvalidPayload(
            "Assigned user does not exist".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let it_asset = ITAsset {
        id,
        asset_name: payload.asset_name,
        asset_type: payload.asset_type,
        purchase_date: payload.purchase_date,
        assigned_to: payload.assigned_to,
        approx_value: payload.approx_value,
        depreciation_rate: payload.depreciation_rate,
    };
    IT_ASSET_STORAGE.with(|storage| storage.borrow_mut().insert(id, it_asset.clone()));
    Ok(it_asset)
}

#[ic_cdk::query]
fn get_it_assets() -> Result<Vec<ITAsset>, Message> {
    IT_ASSET_STORAGE.with(|storage| {
        let it_assets: Vec<ITAsset> = storage
            .borrow()
            .iter()
            .map(|(_, it_asset)| it_asset.clone())
            .collect();

        if it_assets.is_empty() {
            Err(Message::NotFound("No IT assets found".to_string()))
        } else {
            Ok(it_assets)
        }
    })
}

#[ic_cdk::query]
fn get_it_asset_by_id(id: u64) -> Result<ITAsset, Message> {
    IT_ASSET_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, it_asset)| it_asset.id == id)
            .map(|(_, it_asset)| it_asset.clone())
            .ok_or(Message::NotFound("IT asset not found".to_string()))
    })
}

// Function to calculate depreciation
#[ic_cdk::update]
fn calculate_depreciation(payload: CalculateDepreciationPayload) -> Result<f64, Message> {
    IT_ASSET_STORAGE.with(|storage| {
        if let Some(it_asset) = storage.borrow().get(&payload.it_asset_id) {
            let years = payload.years as f64;
            let depreciation_rate = it_asset.depreciation_rate / 100.0;
            let depreciation = (1.0 - depreciation_rate).powf(years);
            let current_value = 1000.0; // Assuming the initial value of the asset is $1000
            let value = current_value * depreciation;
            Ok(value)
        } else {
            Err(Message::NotFound("IT asset not found".to_string()))
        }
    })
}
// Helper function to get the current time
fn current_time() -> u64 {
    time()
}

// Export the candid functions
ic_cdk::export_candid!();
