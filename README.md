
# IT Support System

This project contains an IT Support System designed to manage IT assets, support tickets, and users. The system allows the creation and management of users, IT assets, and support tickets, providing a structured approach to IT support within an organization.

## Features

### User Management

- **Create User**: Add new users with specific roles.
- **View Users**: Retrieve a list of all registered users.
- **Get User by ID**: Fetch user details using a unique identifier.

### IT Asset Management

- **Create IT Asset**: Register new IT assets with details such as name, type, purchase date, assigned user, and depreciation rate.
- **View IT Assets**: Retrieve a list of all registered IT assets.
- **Get IT Asset by ID**: Fetch IT asset details using a unique identifier.
- **Calculate Depreciation**: Compute the current value of an IT asset based on its depreciation rate and the number of years since purchase.

### Support Ticket Management

- **Create Ticket**: Open new support tickets with details like title, description, and priority.
- **Assign Ticket**: Assign tickets to specific IT support users.
- **Update Ticket Status**: Change the status of a ticket (e.g., Open, In Progress, Closed).
- **Add Ticket Comment**: Add comments to tickets for better communication and tracking.
- **View Tickets**: Retrieve a list of all registered tickets.
- **Get Ticket by ID**: Fetch ticket details using a unique identifier.

## Usage

This system is designed for internal use by IT support teams to streamline their operations. It supports role-based access, ensuring that only authorized users can perform certain actions, such as creating IT assets or assigning tickets.

### Roles

- **Admin**: Has full access to the system, including user and asset management.
- **IT Support**: Can manage IT assets and handle support tickets.
- **User**: Can create tickets and view their status.

### Ticket Statuses

- **Open**: Initial status when a ticket is created.
- **In Progress**: Indicates that work has started on the ticket.
- **Closed**: Indicates that the ticket has been resolved.

### Asset Types

- **Laptop**
- **Desktop**
- **Monitor**
- **Printer**
- **Scanner**
- **Other**: Any other type of IT asset.

## Structure

The system is built with a modular design, ensuring that different components, such as user management, asset management, and ticket management, are independent and easily maintainable. Data is stored and managed using a stable structure, ensuring reliability and scalability.

## Export Candid Functions
The platform exports candid functions for interaction with the Internet Computer.




## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```

## How to Contribute

If you'd like to contribute to this project, please fork the repository, make your changes, and submit a pull request. I welcome improvements and new features that can enhance the functionality of the IT Support System.
