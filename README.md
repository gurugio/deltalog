

# How to create the example parquet file from the JSON file with DuckDB
```
gurugio@gioh-laptop:~/study/deltalog$ ./tmp/duckdb 
v0.9.2 3c695d7ba9
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
D create table logtable (logtime DATETIME, hostname TEXT, message TEXT);
D copy logtable from 'example.json';
D select * from logtable;
┌─────────────────────┬─────────────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│       logtime       │  hostname   │                                                     message                                                      │
│      timestamp      │   varchar   │                                                     varchar                                                      │
├─────────────────────┼─────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 2023-12-03 11:49:40 │ gioh-laptop │ dbus-daemon[1937]: [session uid=1000 pid=1937] Successfully activated service 'org.freedesktop.FileManager1'     │
│ 2023-12-03 11:49:40 │ gioh-laptop │ gnome-shell[2164]: Window manager warning: Overwriting existing binding of keysym 32 with keysym 32 (keycode b). │
│ 2023-12-03 11:49:40 │ gioh-laptop │ gnome-shell[2164]: Window manager warning: Overwriting existing binding of keysym 31 with keysym 31 (keycode a). │
│ 2023-12-03 11:49:40 │ gioh-laptop │ gnome-shell[2164]: Window manager warning: Overwriting existing binding of keysym 35 with keysym 35 (keycode e). │
│ 2023-12-03 11:49:40 │ gioh-laptop │ dbus-daemon[1937]: [session uid=1000 pid=1937] Successfully activated service 'org.gnome.ArchiveManager1'        │
│ 2023-12-04 11:49:40 │ gioh-laptop │ gnome-shell[2164]: DING: Detected async api for thumbnails                                                       │
│ 2023-12-04 11:49:40 │ gioh-laptop │ gnome-shell[2164]: DING: GNOME nautilus 42.6                                                                     │
│ 2023-12-05 11:49:50 │ gioh-laptop │ nautilus[157120]: Could not delete '.meta.isrunning': No such file or directory                                  │
│ 2023-12-05 11:51:03 │ gioh-laptop │ google-chrome.desktop[3242]: Fontconfig error: Cannot load default config file: No such file: (null)             │
│ 2023-12-05 12:05:01 │ gioh-laptop │ CRON[160237]: (root) CMD (command -v debian-sa1 > /dev/null && debian-sa1 1 1)                                   │
├─────────────────────┴─────────────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 10 rows                                                                                                                                    3 columns │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
D copy (select * from logtable) to 'example.parquet' (format parquet);
D 
```

# How to read the example parquet file with DuckDB
```
gurugio@gioh-laptop:~/study/deltalog$ ./tmp/duckdb 
v0.9.2 3c695d7ba9
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
D create table logtable as select * from read_parquet('example.parquet');
D select * from logtable;
┌─────────────────────┬─────────────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│       logtime       │  hostname   │                                                     message                                                      │
│      timestamp      │   varchar   │                                                     varchar                                                      │
├─────────────────────┼─────────────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 2023-12-03 11:49:40 │ gioh-laptop │ dbus-daemon[1937]: [session uid=1000 pid=1937] Successfully activated service 'org.freedesktop.FileManager1'     │
│ 2023-12-03 11:49:40 │ gioh-laptop │ gnome-shell[2164]: Window manager warning: Overwriting existing binding of keysym 32 with keysym 32 (keycode b). │
│ 2023-12-03 11:49:40 │ gioh-laptop │ gnome-shell[2164]: Window manager warning: Overwriting existing binding of keysym 31 with keysym 31 (keycode a). │
│ 2023-12-03 11:49:40 │ gioh-laptop │ gnome-shell[2164]: Window manager warning: Overwriting existing binding of keysym 35 with keysym 35 (keycode e). │
│ 2023-12-03 11:49:40 │ gioh-laptop │ dbus-daemon[1937]: [session uid=1000 pid=1937] Successfully activated service 'org.gnome.ArchiveManager1'        │
│ 2023-12-04 11:49:40 │ gioh-laptop │ gnome-shell[2164]: DING: Detected async api for thumbnails                                                       │
│ 2023-12-04 11:49:40 │ gioh-laptop │ gnome-shell[2164]: DING: GNOME nautilus 42.6                                                                     │
│ 2023-12-05 11:49:50 │ gioh-laptop │ nautilus[157120]: Could not delete '.meta.isrunning': No such file or directory                                  │
│ 2023-12-05 11:51:03 │ gioh-laptop │ google-chrome.desktop[3242]: Fontconfig error: Cannot load default config file: No such file: (null)             │
│ 2023-12-05 12:05:01 │ gioh-laptop │ CRON[160237]: (root) CMD (command -v debian-sa1 > /dev/null && debian-sa1 1 1)                                   │
├─────────────────────┴─────────────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ 10 rows                                                                                                                                    3 columns │
└──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘
D 
```