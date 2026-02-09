# Application's parameters


```bash
~$ /opt/home_automation/solar_boiler_monitor --user UserOne --boiler_id SC1Z202859571 --boiler_hostname 169.254.0.125 --reading_values_delay 30000 --log_level debug --dry_running true
[19:28:50 INFO home_automation] Config:
             user_id=[util]
  password_file_path=[/opt/solar_boiler_monitor/.boiler_password.txt]
           boiler_id=[SC1Z202859571]
     boiler_hostname=[169.254.0.125]
reading_values_delay=[30000] milliseconds
           log_level=[debug]
         dry_running=[true]
```

```bash
~$ home_automation --help
--help : display these explainatiions 
--user_id : the user id used to be connected to the http server of you boiler (e.g. util) 
--password_file_path : if the encrypted password isn't in the file "~/.boiler_password.txt", give the path of the file containing the password. 
--boiler_id : the id of you boiler installation. (e.g. SC1Z202859571)
--boiler_hostname : the hostname or the ip address of the boiler in your home lan (e.g. 169.254.0.125)
--reading_values_delay : the delay between each reading ( e.g. 60000, default = 30000)
--log_level : the log level (e.g. error | warn | info | debug)
--dry_running : run the monitor without sending the http requests to the boiler (e.g. true | false, default = false).
```




```txt
Config:
             user_id=[util]
  password_file_path=[/opt/solar_boiler_monitor/.boiler_password.txt]
           boiler_id=[SC1Z202859571]
     boiler_hostname=[169.254.0.125]
reading_values_delay=[30000] milliseconds
           log_level=[debug]
         dry_running=[true]
```


```bash
$ /opt/home_automation/solar_boiler_monitor --version
solar_boiler_monitor 1.0
```




```bash
$ /opt/home_automation/solar_boiler_monitor --help
    This program monitors a solar boiler and sends data to a server.
    It can run in dry-run mode to test without sending HTTP requests to the solar boiler.
    

Usage: home_automation [OPTIONS] --user-id <USER_ID> --boiler-id <BOILER_ID> --boiler-hostname <BOILER_HOSTNAME>

Options:
      --user-id <USER_ID>
          The user ID used to connect to the boiler's HTTP server

      --password-file-path <PASSWORD_FILE_PATH>
          Path to the file containing the encrypted password
          
          [default: ./.boiler_password.txt]

      --boiler-id <BOILER_ID>
          The ID of your boiler installation

      --boiler-hostname <BOILER_HOSTNAME>
          The hostname or IP address of the boiler in your home LAN

      --reading-values-delay <READING_VALUES_DELAY>
          The delay between each reading, in milliseconds
          
          [default: 30000]

      --log-level <LOG_LEVEL>
          The log level
          
          [default: info]

      --dry-running
          Run the monitor without sending HTTP requests to the boiler

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Process finished with exit code 0
```


