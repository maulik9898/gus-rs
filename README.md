# Git User Switch (gus) - Simplify Git User Switching with Rust

<p align="center">
  <img width="500" height="300" src="docs/gus.gif"/>
</p>

Git User Switch (gus) is a command-line tool written in Rust using the `clap-rs` library that simplifies the process of switching between Git user profiles. If you frequently work with multiple Git profiles or accounts, this tool can help streamline your workflow by providing an intuitive interface for managing and activating different profiles.

## Features

- **Simple Interface**: The tool offers a straightforward command-line interface to manage your Git profiles.
- **Profile Listing**: List all available profiles and choose the one you want to activate.
- **Profile Management**: Add, edit, delete, and activate profiles easily.

## Installation

You can install Git User Switch (gus) using the following command:

```bash
cargo install gus-rs
```

## Usage

Switching between Git user profiles is now easier than ever. Use the following commands to manage your profiles:

- **List Profiles and Select**: View all available profiles and choose the one you want to activate.
  ```bash
  gus list
  ```
  <p align="center">
    <img width="500" height="300" src="docs/list.gif"/>
  </p>

- **Add New Profile**: Create a new Git user profile.
  ```bash
  gus add --name JohnDoe --email john@example.com --profile work
  ```
  <p align="center">
    <img width="500" height="300" src="docs/add.gif"/>
  </p>

- **Activate Profile**: Switch to a specific Git user profile.
  ```bash
  gus ac work
  ```

- **Delete Profile**: Remove an existing Git user profile.
  ```bash
  gus delete
  ```
  <p align="center">
    <img width="500" height="300" src="docs/delete.gif"/>
  </p>

- **Edit Profile**: Modify the details of an existing Git user profile.
  ```bash
  gus edit
  ```

- **Help**: Access detailed help information for each command.
  ```bash
  gus help
  ```

## Contributing

Contributions are welcome! If you find any issues, have suggestions, or want to contribute, feel free to open an issue or submit a pull request. 

## License

This project is licensed under the [MIT License](LICENSE).

