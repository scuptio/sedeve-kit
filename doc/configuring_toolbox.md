Before writing a TLA+ specification,
you need to configure the [TLA+ toolbox](https://lamport.azurewebsites.net/tla/toolbox.html) to install additional dependent community modules.

Follow these steps to set it up:

1. Download the [SedeveModules jar](https://github.com/scuptio/SedeveModules/releases).
   We developed TLA+ modules
   [StateDB](https://github.com/scuptio/SedeveModules/blob/master/modules/StateDB.tla),
   [GenID](https://github.com/scuptio/SedeveModules/blob/master/modules/GenID.tla) .

   Download the [SQLite JDBC driver jar](https://github.com/xerial/sqlite-jdbc/releases/download/3.45.3.0/sqlite-jdbc-3.45.3.0.jar)

2. In tla+ toolbox, go to

   File -> Preferences -> TLA+ Preference

   Specify TLA+ library path option of *SedeveModules*,  *SQLite JDBC driver*


   <img src="../doc/figure/set_library_path_locations.png" width="50%" height="50%">
