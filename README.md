# Pi_TCP

## Overview
This repository was created as a developmental platform for receiving database information via TCP connection. **Please note that this repository's functionality has been replaced by Pi_Data_Receiver.**

## Historical Purpose
Pi_TCP was designed to handle TCP connections for database transfers from a Raspberry Pi system. It served as a proof of concept and development environment before the more robust Pi_Data_Receiver was implemented.

## Configuration
The repository uses a configuration file (`config.ini`) with the following structure:

```ini
[server]
ip = 0.0.0.0     ; Server IP address
port = 7878            ; TCP port to listen on
max_retries = 3        ; Maximum number of connection retries
retry_delay = 2        ; Delay between retries in seconds
```

## Deprecation Notice
This repository is no longer actively maintained. For current implementation, please refer to the Pi_Data_Receiver repository which provides improved functionality and reliability.

## License Notice
To apply the Apache License to your work, attach the following boilerplate notice. The text should be enclosed in the appropriate comment syntax for the file format. We also recommend that a file or class name and description of purpose be included on the same "printed page" as the copyright notice for easier identification within third-party archives.

    Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
    Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
    
    http://www.apache.org/licenses/LICENSE-2.0
    Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

