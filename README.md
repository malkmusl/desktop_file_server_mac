# Desktop File Server Mac

[![Malkmusl's Proprietary License Agreement v1.1](https://img.shields.io/badge/License-Malkmusl's%20Proprietary%20License%20Agreement%20v1.1-red.svg?style=for-the-badge)](LICENSE.md)

![Languages Badge](https://img.shields.io/badge/Languages-818080?style=for-the-badge)  ![Rust Badge](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

[![Buy me a coffee on Ko-fi](https://img.shields.io/badge/Buy%20me%20a%20coffee-Ko--fi-%23FF5E5B.svg?style=for-the-badge)](https://ko-fi.com/your_username)  [![Support me on Patreon](https://img.shields.io/badge/Support%20me%20on-Patreon-orange.svg?style=for-the-badge)](https://www.patreon.com/your_username)

## Table of Contents

- [Project Overview](#project-overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Project Overview

This code is a Rust program that acts as a client for a desktop file server running on Linux. It connects to the Linux server and sends application information to create desktop files. Desktop files are used in Linux environments to launch applications with graphical user interfaces.

## Features

The client code connects to the Linux server and sends application information to create desktop files. The important functions in the code are:

- `main`: Establishes a connection to the Linux server and sends application information.
- `get_runnable_programs`: Retrieves a list of runnable programs on macOS or a demo app for testing on Mac.
- `is_executable`: Checks if a file is executable based on its permissions.
- `send_runnable_programs`: Sends the runnable programs to the Linux server.

## Installation

To install and use this client code, follow these steps:

1. Go to the [Releases](https://github.com/your_username/your_project/releases) page of the repository.

2. Locate the latest release and download the binary file for your macOS system. The binary file will be named something like `client_binary`.

3. Open a terminal and navigate to the directory where the binary file is located.

4. Make the binary file executable by running the following command:

   ```shell
   chmod +x client_binary
   ```

   Replace `client_binary` with the actual name of the downloaded binary file.

5. Run the client program by executing the binary file:

   ```shell
   ./client_binary
   ```

   Replace `client_binary` with the actual name of the binary file.

6. The client program will connect to the Linux server running on the specified address (e.g., `127.0.0.1:1234`).

7. The client will retrieve a list of runnable programs on macOS or a demo app for testing on Mac.

8. The client will send the application information to the Linux server.

9. The Linux server will receive the information and create desktop files based on that data.

If you encounter any issues during installation or have any questions, please don't hesitate to contact us (see the [Contact](#contact) section below).

Please ensure that you have read and agreed to the terms and conditions of the Malkmusl's Proprietary License Agreement v1.0 before using the software. The license agreement can be found in the [LICENSE.md](LICENSE.md) file.

Note: The binary file is provided for transparency and convenience. If you prefer to build the client code from source, you can follow the instructions in the [Usage](#usage) section to build and run the code using Rust.

If you encounter any issues or have any questions, please don't hesitate to contact us (see the [Contact](#contact) section below).

Please remember that the use of this software is subject to the terms and conditions specified in the Malkmusl's Proprietary License Agreement v1.0.

## Usage

To use the desktop file client program, follow these steps:

1. Ensure that you have read and agreed to the terms and conditions of the Malkmusl's Proprietary License Agreement v1.0 before using the software. The license agreement can be found in the [LICENSE.md](LICENSE.md) file.

2. Download the binary file for your macOS system from the [Releases](https://github.com/your_username/your_project/releases) page of the repository.

3. Open a terminal and navigate to the directory where the binary file is located.

4. Make the binary file executable by running the following command:

   ```shell
   chmod +x client_binary
   ```

   Replace `client_binary` with the actual name of the downloaded binary file.

5. Run the client program by executing the binary file:

   ```shell
   ./client_binary
   ```

   Replace `client_binary` with the actual name of the binary file.

6. The client program will connect to the Linux server running on the specified address (e.g., `127.0.0.1:1234`).

7. The client will retrieve a list of runnable programs on macOS or a demo app for testing on Mac.

8. The client will send the application information to the Linux server.

9. The Linux server will receive the information and create desktop files based on that data.

Note: Ensure that the Linux server is running and listening for connections from the macOS client.

If you encounter any issues or have any questions, please don't hesitate to contact us (see the [Contact](#contact) section below).

Please remember that the use of this software is subject to the terms and conditions specified in the Malkmusl's Proprietary License Agreement v1.0.

To use the desktop file client program, follow these steps:

1. Ensure that you have read and agreed to the terms and conditions of the Malkmusl's Proprietary License Agreement v1.0 before using the software. The license agreement can be found in the [LICENSE.md](LICENSE.md) file.

2. Download the source code from the repository or clone the repository to your local machine.

3. Ensure that you have Rust installed on your macOS system. You can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

4. Open a terminal and navigate to the directory where the source code is located.

5. Build the project by running the following command:

   ```shell
   cargo build --release
   ```

6. Once the build process is complete, you will find the executable file in the `target/release` directory.

7. Run the client program by executing the binary file:

   ```shell
   ./target/release/client_binary
   ```

   Replace `client_binary` with the actual name of the binary file.

8. The client program will connect to the Linux server running on the specified address (e.g., `127.0.0.1:1234`).

9. The client will retrieve a list of runnable programs on macOS or a demo app for testing on Mac.

10. The client will send the application information to the Linux server.

11. The Linux server will receive the information and create desktop files based on that data.

Note: Ensure that the Linux server is running and listening for connections from the macOS client.

If you encounter any issues or have any questions, please don't hesitate to contact us (see the [Contact](#contact) section below).

Please remember that the use of this software is subject to the terms and conditions specified in the Malkmusl's Proprietary License Agreement v1.0.

## Contributing

Let others know how they can contribute to your project. Provide guidelines for code formatting, issue reporting, and pull requests. Mention any specific development setup or testing requirements.

Contributions to this project are subject to review and approval by the Malkmusl team. By submitting a contribution, you agree to comply with the Malkmusl's Proprietary License and grant Malkmusl the right to use and distribute your contributions under the terms of the license.

Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for detailed guidelines on how to contribute.

## License

This project is proprietary and subject to the Malkmusl's Proprietary License. The source code and any associated files are the property of Malkmusl and cannot be distributed or modified without explicit permission from the owner. You are only granted the right to use the software as provided. See the [LICENSE.md](LICENSE.md) file for more details.

## Contact

Provide contact information for users to reach out to you or your team. This can include email addresses, links to social media profiles, or any other preferred communication channels.
