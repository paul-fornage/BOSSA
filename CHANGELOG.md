# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v2.0.0 (2023-10-12)

<csr-id-a96c5a09328b7839507bda60556290d7092345c7/>
<csr-id-2ae59e09ba3aea699eebb35552d55e743ce25908/>
<csr-id-25e8fdcfbcaf839b954957b1762e1ce098167a30/>
<csr-id-62b1e65a2762baf607208c1dd84cd489e033f9da/>
<csr-id-f83f741d07ef2d4fb10404a74bf2b9a9aa01ec09/>

### New Features

 - <csr-id-8e233bacc80b8bcca9a07821ea2a1891a26422b9/> Initial bossa library and bossac utility
   - CXX Rust wrapper around bossa so it can be statically linked library
   into rust utilities (this is useful for Windows applications rather
   than relying on external processes).
   * To get the rust wrapper to work some C++ changes were necessary to
   cleanup the api. This probably would have been easier if the wider
   C++ apis were const'd correctly but I err'd on the side of minimal
   changes.
* Keep Linux at 3.0 for Ubuntu GitHub Action
 - <csr-id-8b29f5ac3b18d59edecc904fcfae612ebab78bcc/> Initial bossa library and bossac utility
   - CXX Rust wrapper around bossa so it can be statically linked library
     into rust utilities (this is useful for Windows applications rather
     than relying on external processes).
     * To get the rust wrapper to work some C++ changes were necessary to
       cleanup the api. This probably would have been easier if the wider
       C++ apis were const'd correctly but I err'd on the side of minimal
       changes.
   - bossac tries to be as close as possible to the C++ version of bossac
     (including cli args)
   - Does not expose the complete bossa library api (only those necessary
     for bossac)
   - An equivalent bossash should be possible, but I don't really use it so
     leaving it open to future contributions.
   - The bossa WxWidgets utility is more difficult, but might be possible
     with https://github.com/kenz-gelsoft/wxRust2.
     Personally, I would probably recommend https://github.com/KDAB/cxx-qt.
     But I don't really use either so I'll leave these as open to future
     contributions.
   - The existing C++ binaries should continue to function/build
   - Fix formbuilder warning (info)
   - Update wxWidgets mostly to 3.2
     * Keep Linux at 3.0 for Ubuntu GitHub Action

### Other

 - <csr-id-a96c5a09328b7839507bda60556290d7092345c7/> Add missing include
   select() is declared in <sys/select.h>. This commit adds the missing include
 - <csr-id-2ae59e09ba3aea699eebb35552d55e743ce25908/> Call tcdrain() to write serial data
 - <csr-id-25e8fdcfbcaf839b954957b1762e1ce098167a30/> bugs.debian.org/953068
   src/Command.cpp
   In Ubuntu, the latest version of bossa has failed to build on ppc64el
   because the ppc64el architecture in Ubuntu uses -O3 as a default
   optimization, which exposes a number of uninitialized variables in the code.
   
   In reality none of these variables are used uninitialized, but the compiler
   can't /prove/ that, so it errors out.
 - <csr-id-62b1e65a2762baf607208c1dd84cd489e033f9da/> Set DTR behavior explicitly
   Some serial terminal applications apparently change the default DTR behavior ("turn off and keep off"); this causes BOSSA to fail, e.g. on the Arduino Due, until the parameter is reset or the serial device is disconnected and reconnected. Make BOSSA set this parameter explicitly.
 - <csr-id-f83f741d07ef2d4fb10404a74bf2b9a9aa01ec09/> add -V/--version command-line option
     * It's helpful to get the version number without the whole verbose
       help text. Add -V/--version option for this like many other utilities.
     * Move help text for -h/-V to the bottom of the list so they're not
       mixed in with other behavior options.
     * Exit with a success return code (0) instead of failure (1) on -h/-V,
       as is conventional.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 29 commits contributed to the release over the course of 1873 calendar days.
 - 1876 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Initial bossa library and bossac utility ([`8b29f5a`](https://github.com/kiibohd/BOSSA/commit/8b29f5ac3b18d59edecc904fcfae612ebab78bcc))
    - Add missing include ([`a96c5a0`](https://github.com/kiibohd/BOSSA/commit/a96c5a09328b7839507bda60556290d7092345c7))
    - Add support for ATSAMR34J18 ([`599f5ee`](https://github.com/kiibohd/BOSSA/commit/599f5ee8072ea977f8e2fb5b9e1453e0c4686cb9))
    - Update Device.cpp ([`a26ab38`](https://github.com/kiibohd/BOSSA/commit/a26ab384cdb7180f0a830b95696cdffb67c777dc))
    - Update Device.cpp ([`8407799`](https://github.com/kiibohd/BOSSA/commit/8407799a6adb8eb844b7696ac4133b06e450b5f4))
    - Call tcdrain() to write serial data ([`2ae59e0`](https://github.com/kiibohd/BOSSA/commit/2ae59e09ba3aea699eebb35552d55e743ce25908))
    - Fixed ram usage start for 8K devices ([`d45c9cb`](https://github.com/kiibohd/BOSSA/commit/d45c9cbf771a577916d15e9d2b9c043cb6be4b0d))
    - Added support for ATSAMC21 devices ([`b1a4900`](https://github.com/kiibohd/BOSSA/commit/b1a4900a1d7da58ed18c2fb1c15101a5b4250b5c))
    - Reverted unused files ([`61dd141`](https://github.com/kiibohd/BOSSA/commit/61dd1417e7ba6d111bb5cf6182129deb2963b879))
    - Removed changes to unused files in preparation for pull request ([`5c77a71`](https://github.com/kiibohd/BOSSA/commit/5c77a71304f437028144ff757fc6d566b166e112))
    - Ported Bossa to 64bit MSVC 2019 ([`25d61eb`](https://github.com/kiibohd/BOSSA/commit/25d61ebb71731590a66fcf3ea2fbde8273daac73))
    - Bugs.debian.org/953068 ([`25e8fdc`](https://github.com/kiibohd/BOSSA/commit/25e8fdcfbcaf839b954957b1762e1ce098167a30))
    - Set DTR behavior explicitly ([`62b1e65`](https://github.com/kiibohd/BOSSA/commit/62b1e65a2762baf607208c1dd84cd489e033f9da))
    - ATSAM4SD16 chip ID's changed ([`4cfaea5`](https://github.com/kiibohd/BOSSA/commit/4cfaea57e9bf3ec5beb7257982876aa1d728666e))
    - Fixed wrong address calculation in mwf and mww commands in Bossash. ([`24bcd56`](https://github.com/kiibohd/BOSSA/commit/24bcd5697f73de7e988392e51b8332a94375bd54))
    - Wait for EEFC flash to complete operations before resetting ([`f86c8b5`](https://github.com/kiibohd/BOSSA/commit/f86c8b5060836897e7453f360e0089b81fe1143f))
    - Fixing typo for ATSAM4S4(ABC) lock regions ([`68ba37d`](https://github.com/kiibohd/BOSSA/commit/68ba37de074007f1d6e14ec6697866530e82b428))
    - Adding note for flash write failures ([`599fae1`](https://github.com/kiibohd/BOSSA/commit/599fae16b302879c51bffb4361ee142525cfd5f5))
    - Adding support for EefcFlash unique ids ([`3dcfb64`](https://github.com/kiibohd/BOSSA/commit/3dcfb64135b2b64807efa65f3c44fb9876be0067))
    - Kiibohd UI Tweaks ([`f3b4bc4`](https://github.com/kiibohd/BOSSA/commit/f3b4bc423e5fc99997a15c37b9903e9dfc2ef0bc))
    - Merge pull request #78 from aswild/arduino-reset ([`3532de8`](https://github.com/kiibohd/BOSSA/commit/3532de82efd28fadbabc2b258d84dddf14298107))
    - Remove legacy file ([`b42fc6d`](https://github.com/kiibohd/BOSSA/commit/b42fc6de6969d1dd1bc2ff3b3adc91581988de5e))
    - Merge pull request #100 from cmaglie/fix-crc ([`724e9cf`](https://github.com/kiibohd/BOSSA/commit/724e9cf15872786e084f405a1c941ba3245894cd))
    - Fix CRC calculation on verify for crc-capable bootloaders ([`cd29178`](https://github.com/kiibohd/BOSSA/commit/cd291783e3fb4e387d1ec09e7e8d487ddb7d8563))
    - Merge pull request #92 from aswild/bossac-version-option ([`8202074`](https://github.com/kiibohd/BOSSA/commit/8202074d53ba666a7bbe9def780a9a9f78a4b140))
    - Merge pull request #93 from cchaloin/patch-1 ([`ae2c04d`](https://github.com/kiibohd/BOSSA/commit/ae2c04d9bfa51f033e9220242b413d493c011b48))
    - Update Device.cpp ([`1d9cc6c`](https://github.com/kiibohd/BOSSA/commit/1d9cc6c06035fb8daacb470e43753f5714e59ae7))
    - Add -V/--version command-line option ([`f83f741`](https://github.com/kiibohd/BOSSA/commit/f83f741d07ef2d4fb10404a74bf2b9a9aa01ec09))
    - Fixed bug in bossash mrf command submitted by fjhenigman ([`260d9e8`](https://github.com/kiibohd/BOSSA/commit/260d9e854f9af9517465bb05cd5c0017e0788a27))
</details>

<csr-unknown>
bossac tries to be as close as possible to the C++ version of bossac(including cli args)Does not expose the complete bossa library api (only those necessaryfor bossac)An equivalent bossash should be possible, but I don’t really use it soleaving it open to future contributions.The bossa WxWidgets utility is more difficult, but might be possiblewith https://github.com/kenz-gelsoft/wxRust2.Personally, I would probably recommend https://github.com/KDAB/cxx-qt.But I don’t really use either so I’ll leave these as open to futurecontributions.The existing C++ binaries should continue to function/buildFix formbuilder warning (info)Update wxWidgets mostly to 3.2<csr-unknown/>

## v1.9.1 (2018-08-23)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 59 commits contributed to the release over the course of 575 calendar days.
 - 706 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fixed problem with comparing lock region count in Eefc flashes. ([`1ecb5eb`](https://github.com/kiibohd/BOSSA/commit/1ecb5eb795e2b0c7087145057720fac530a6e75b))
    - Wait after Arduino 1200 baud reset to allow chip to reboot and re-enumerate ([`c704f9f`](https://github.com/kiibohd/BOSSA/commit/c704f9f2442bc49aa05c9f268637734d2c6712f0))
    - Added SAM Cortex families to the device reset. ([`b176eee`](https://github.com/kiibohd/BOSSA/commit/b176eeef918fc810045c832348590595120187b4))
    - Merge pull request #49 from M-Bab/master ([`9e6ce5e`](https://github.com/kiibohd/BOSSA/commit/9e6ce5e9f11cabf8bf8d91d0a0b7a95e7e7abf28))
    - Added families for the SAME5x. ([`a0b8421`](https://github.com/kiibohd/BOSSA/commit/a0b842117da15e77d5d3a3d546cbf48aedbbb712))
    - Added SAML21 devices.  (untested) ([`6f64610`](https://github.com/kiibohd/BOSSA/commit/6f646108820260836a4dcf386dcde210041663f6))
    - Added SAM S70/V70/V71 devices.  (untested) ([`ebbc97d`](https://github.com/kiibohd/BOSSA/commit/ebbc97de5b537f0c78e9cc57716d3c1fc0835846))
    - Clear errno before strtol call in case it was already set. ([`906ecd1`](https://github.com/kiibohd/BOSSA/commit/906ecd1da0fa2d578c13174e9413212e39003db3))
    - Worked around limitation in mac C++ library by passing a reference to an unique_ptr rather than passing it back by value. ([`b8cee12`](https://github.com/kiibohd/BOSSA/commit/b8cee12d34db0e7f29536aaf9d90d7aecc1a23f3))
    - Don't add the BMP handler since wxWidgets automatically adds it. ([`6b23b94`](https://github.com/kiibohd/BOSSA/commit/6b23b942a0bd7e9795e69f2612860a48a4deaa80))
    - Merge pull request #67 from kiibohd/sam4sd ([`0146132`](https://github.com/kiibohd/BOSSA/commit/0146132868e7060179b55ff36809f6aa698b10a1))
    - Merge pull request #68 from kiibohd/errno ([`4cd1323`](https://github.com/kiibohd/BOSSA/commit/4cd132319a49bc027dd8e9d24c5101b3850c054b))
    - Bumped copyrights to 2018 ([`d480a1e`](https://github.com/kiibohd/BOSSA/commit/d480a1ed34f4d08ac4822fe3aa5d1149b53b40c8))
    - Added support for the SAMD51.  Rewrote most of the D21 support since the existing code was not working.  Refactored the flash options to defer writing them until an explicit write is called which avoids writing the same user page over and over for the D21 and D51. ([`f7a65bb`](https://github.com/kiibohd/BOSSA/commit/f7a65bb28cbfd8be116f75ee1d799747e08a8274))
    - Extend SAM4S Family ([`2d5b6c5`](https://github.com/kiibohd/BOSSA/commit/2d5b6c5c16edb7a1bda065ab1f8854a188790051))
    - Fixing strtol macOS compilation error ([`673581e`](https://github.com/kiibohd/BOSSA/commit/673581eac840740c3e4d4a3829e6c72650891812))
    - Fixing macOS compilation error ([`5357480`](https://github.com/kiibohd/BOSSA/commit/53574806baf3eec3a90689357bcaf9d1eb414390))
    - All Arduino and AdaFruit SAMD boards with a flash-based bootloader have a bug where reading 64 bytes or more over USB returns corrupted data. This is because the bootloaders try to send 64+ byte USB packets with a pointer into flash instead of SRAM which is not allowed on the SAMD. Instead we detect the Arduino bootloader and break read transactions into multiple requests no larger than 63 bytes. ([`df65c45`](https://github.com/kiibohd/BOSSA/commit/df65c459e5bb1022fa458b5ac7f15e744a8fb426))
    - Refactored NvmFlash class to be abstract with the registers and commands as variables defined in the subclass.  Changed all the D21 and R21 devices to instantiate a NvmFlashD2x subclass.  This was all done in anticipation of adding support for the D51. ([`66dd8cf`](https://github.com/kiibohd/BOSSA/commit/66dd8cf9c68386a15e19a90f2a458258a0ce193f))
    - Cleaned up some comments and help text about the bootloader ([`8a47bf0`](https://github.com/kiibohd/BOSSA/commit/8a47bf0906e512cefc075f9e0678ab2a043f96b6))
    - Scanf is an abomination ([`d50923d`](https://github.com/kiibohd/BOSSA/commit/d50923df78a1ca83d0ae14f0b6470b956edb0edd))
    - Removed knowledge of the bootloader size from BOSSA.  That is the user's concern, not BOSSA's.  The flash offset now applies to erases to support not erasing bootloaders or any other code before the offset. The NvmFlash now supports on the fly auto-erase like the other flashes. The GUI now only attempts to set flags if the flash indicates it can. ([`dbdd088`](https://github.com/kiibohd/BOSSA/commit/dbdd088909c57bdf9aee191720bf6c6159217e22))
    - Added the balance of the D21 and R21 families based on information in the datasheets.  Note these are largely untested. ([`deecb80`](https://github.com/kiibohd/BOSSA/commit/deecb80d3f7fcb251ad7012a54656651fbcbf00e))
    - Fixed warning about undersized sprintf buffer ([`86d82af`](https://github.com/kiibohd/BOSSA/commit/86d82af4af6c6ff9f54bf7302187654db2e9caa6))
    - If the Samba version() method throws an exception during the version check in the init() method, then catch and return an error so the exception is not propogated out. ([`fbca22d`](https://github.com/kiibohd/BOSSA/commit/fbca22d36b351a6b771cd34a0b632d890c2db0ab))
    - Cleaned up warnings from Arduino reset PR ([`2c1de9b`](https://github.com/kiibohd/BOSSA/commit/2c1de9baaa07a18839a39c9de73d13cbccd36d00))
    - Merge branch 'master' of github.com:shumatech/BOSSA ([`6fcc373`](https://github.com/kiibohd/BOSSA/commit/6fcc373244812f8b1db618ac539bcfbd8ffeceac))
    - Fixed bug submitted by cw2 where the size was being stored in the offset variable. ([`7c0fea4`](https://github.com/kiibohd/BOSSA/commit/7c0fea4877d004da4d8bd085d19c8204ae9bcd89))
    - Merge pull request #61 from laf0rge/master ([`a68b549`](https://github.com/kiibohd/BOSSA/commit/a68b549ada1f4262ed057c59f080fa08b2b2b059))
    - Filled out the SAME70 family based on the datasheet (untested).  Fixed up the pull request for coding conventions. ([`8aced2f`](https://github.com/kiibohd/BOSSA/commit/8aced2ff7d78d26e19349337e4d5e8a656ff8ceb))
    - Merge branch 'master' of git://github.com/chrishamm/BOSSA ([`481f723`](https://github.com/kiibohd/BOSSA/commit/481f7231af4b943189ab8815c5955f0e7efc1d0a))
    - Added proper include for ioctl() used by the 1200bps hack ([`7924944`](https://github.com/kiibohd/BOSSA/commit/7924944ac20c67835e537ba405ea224b2c244f1e))
    - Merge branch '1200bps' of git://github.com/kevans91/BOSSA ([`0755bf6`](https://github.com/kiibohd/BOSSA/commit/0755bf6c2bacf87f7f585b988b642edaf50c0c86))
    - Added support for SAME70Q21 ([`be9a8b7`](https://github.com/kiibohd/BOSSA/commit/be9a8b77a22a24959ca3bd79f46ee88ce42e1e7a))
    - Add support for SAM3SD8 and SAM3S8 ([`0615b84`](https://github.com/kiibohd/BOSSA/commit/0615b848314e98d497bf7a19c7a1df23401b40b8))
    - Catch exception in Device.cpp->reset which is very likely to be thrown because the serial connection breaks when the CPU is reset. ([`e369a34`](https://github.com/kiibohd/BOSSA/commit/e369a34fa08121052dbe8cdc31401ed87cd6585b))
    - Merge pull request #48 from kevans91/bsd-build ([`2ebf559`](https://github.com/kiibohd/BOSSA/commit/2ebf559da43bb0ea6a3d9a17739458002b51ac52))
    - Remove note about port autodetection ([`a071ba6`](https://github.com/kiibohd/BOSSA/commit/a071ba6bad19f297ef11216e7e2a3146705915a5))
    - Consistency ([`26f61ca`](https://github.com/kiibohd/BOSSA/commit/26f61ca1e23214c43216899a2307e60e6bdb9d21))
    - Add support for setDTR/setRTS to WinSerialPort ([`648c7d1`](https://github.com/kiibohd/BOSSA/commit/648c7d12e5eb751be1debd79701a6ee63be25aa5))
    - Fix BSD build -- seems to have been a typo, restore BSDPortFactory::end() to pre-ee5a651 behavior and add BSDPortFactory::def() ([`cac3432`](https://github.com/kiibohd/BOSSA/commit/cac3432a7560e06dd95e43bb080a624d772219bb))
    - Address concerns by @shumatech, properly fix merge conflicts ([`f721204`](https://github.com/kiibohd/BOSSA/commit/f721204a09042fd77aca890cbbacb0f3901cb70c))
    - Merge branch 'master' into 1200bps ([`662d60c`](https://github.com/kiibohd/BOSSA/commit/662d60cfd628a124d6a8445d5ce653aa454a8005))
    - Fixed some minor OSX build issues. ([`3be622c`](https://github.com/kiibohd/BOSSA/commit/3be622ca0aa6214a2fc51c1ec682c4a58a423d62))
    - Added SAM3N0 devices (A-C) ([`64f5d5e`](https://github.com/kiibohd/BOSSA/commit/64f5d5ea6cbc962f8e5c41983d3bd2755c29ec7e))
    - Added detection of SAMD21x18 and SAMR21x18 families.  UNTESTED!!! ([`a62710e`](https://github.com/kiibohd/BOSSA/commit/a62710e3d0e861c2abcb33d4060352df69e783d0))
    - Fixes issue #37 where the input pointer was compared to the null character. ([`3ace00e`](https://github.com/kiibohd/BOSSA/commit/3ace00ee9a5a1372c321f4b97b941eb7c040ef80))
    - Refactored to encapsulate all device-specific knowledge into the Device class including flash creation to prevent Samba from being a dumping ground for miscellaneous device features. ([`ee5a651`](https://github.com/kiibohd/BOSSA/commit/ee5a651aacaf2c3fbcd539789574618a425d1a06))
    - Fixed problem with lock regions on SAM4E.  Some general cleanup. ([`2615437`](https://github.com/kiibohd/BOSSA/commit/26154375695f345491bba158d57177aa231d6765))
    - Changed info dialog to show a comma separated list of lock regions instead of a grid of check boxes.  This allows for more than 32 lock regions which is needed for some of the newer Atmel devices. ([`c0cfe18`](https://github.com/kiibohd/BOSSA/commit/c0cfe18ef1fb68d4d7df1a1c859117f7eeea7e26))
    - Added support for SAM4E family. ([`3f6fac5`](https://github.com/kiibohd/BOSSA/commit/3f6fac5a54e1337ef5ce0286a345ff1cf89b8520))
    - Slowed down and extended the FSR poll loop.  Added check for flash error in FSR loop.  Added separate timeout exception if FSR loop reaches the end.  Added FSR polling after erase all since it can take upwards of 20 seconds on some SAM4D devices. ([`16087bd`](https://github.com/kiibohd/BOSSA/commit/16087bd4f4fc2035fb5637ef07a9c247a786f47c))
    - Fix lock bits for devices with > 32 lock bits like the SAM4S8/16 ([`3584599`](https://github.com/kiibohd/BOSSA/commit/35845997a28d354d4ff2887bebcd0b51542f47f1))
    - Fix printf format warning ([`012619c`](https://github.com/kiibohd/BOSSA/commit/012619c6e04b4e0b740b5969741fa7fda983de7d))
    - Add support for baud=1200, assert RTS=true, DTR=false for 1200bps erase ([`c71ad6d`](https://github.com/kiibohd/BOSSA/commit/c71ad6d989aea2f69c10c9eb6198c3d83be6e7c7))
    - Add arduino 1200bps erase flag (-a) ([`e9530b8`](https://github.com/kiibohd/BOSSA/commit/e9530b874a588f76ab87b6180177eb7404bb0927))
    - Added offset feature to start read/write/verify at an offest into the flash.  Fixed old devices broken by SAMD changes.  Made Flasher class more generic with an observer pattern so that the GUI BOSSA can use it and simplify further maintenance.  Refactored many of the newer changes to conform better to an OO paradigm. ([`71d14d6`](https://github.com/kiibohd/BOSSA/commit/71d14d6e574ec2415841f26411458d9f06bf53af))
    - Compile for wxWidgets 3.1.  Use Wix 3.10 for Windows installer.  Changed to use unique_ptr instead of auto_ptr.  Regenerated the form code using wxFormBuilder 3.5. ([`dbfbbdd`](https://github.com/kiibohd/BOSSA/commit/dbfbbdde6de52484f3befbe40ad7324ff37b664f))
    - Fixed broken chipID identification for older devices and removed out of place coding style. ([`c626aab`](https://github.com/kiibohd/BOSSA/commit/c626aabfdf0216d9f16a636f118e7b146f4f0269))
</details>

## v1.7.0 (2016-09-16)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 279 calendar days.
 - 317 days passed between releases.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merge branch 'arduino' of https://github.com/arjunak234/BOSSA into arduino ([`a4c17c7`](https://github.com/kiibohd/BOSSA/commit/a4c17c70ad3bc4722e0d903e2923c1f7473f2295))
    - Merge branch 'patch-1' of https://github.com/mfm24/BOSSA into arduino ([`d0b384d`](https://github.com/kiibohd/BOSSA/commit/d0b384d5a7b68a55bc2ef28dd060927c8a2494ae))
    - Merge branch 'arduino-freebsd' of https://github.com/kevans91/BOSSA into arduino ([`d3c658a`](https://github.com/kiibohd/BOSSA/commit/d3c658af7c5008eccc000bbbbeda5e67d20acfb8))
    - Minor typo in SAM R21 chip name (ATSAMR21E18A) ([`3083757`](https://github.com/kiibohd/BOSSA/commit/3083757265944ddcc8fe5c79cbd3fd599fa27ff7))
    - Minor typo in SAM R21 chip name (ATSAMR21E18A) ([`259aa51`](https://github.com/kiibohd/BOSSA/commit/259aa5109fc2b104131b833e9418e1f789a64e41))
    - Bug fix - The Bootloader Size value for the SAM R21 was wrong (0x1000). It has been corrected (0x2000) to work with the zero_osc32k bootloader and atsamr21e18a core ([`e2ed718`](https://github.com/kiibohd/BOSSA/commit/e2ed718e26f263aabd204d6c975b3a3cabb37d06))
    - Allow full path in PosixSerialPort constructor ([`5f3a8e2`](https://github.com/kiibohd/BOSSA/commit/5f3a8e28b51539ca4df06e4a321123e453443b7f))
    - Add FreeBSD support, piggyback off of OpenBSD ([`f1de539`](https://github.com/kiibohd/BOSSA/commit/f1de539863ec3c55a7fac8d4ffb2e557a1acf967))
    - Fix division by zero ([`85a1f17`](https://github.com/kiibohd/BOSSA/commit/85a1f174a297d682ec7cdf84d7daa07f601974c3))
    - Merge branch 'arduino' of https://github.com/shumatech/BOSSA ([`ce82a9f`](https://github.com/kiibohd/BOSSA/commit/ce82a9fac4b916e359fb003ee4d19afc0ff01c90))
</details>

## v1.6.1-arduino (2015-11-03)

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 123 commits contributed to the release over the course of 1682 calendar days.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fixed progress bar ([`293b738`](https://github.com/kiibohd/BOSSA/commit/293b73818e113bf4544bc1aa636495f1b2f0832d))
    - Revert "Restore termios settings after using serial port" ([`226b562`](https://github.com/kiibohd/BOSSA/commit/226b562998299ad016ca73c95981ae00dd2c0b9a))
    - Merge pull request #16 from logos-electromechanical/arduino ([`025bc55`](https://github.com/kiibohd/BOSSA/commit/025bc55ab0bf3651a54dd58a333deb4c5dd23cd6))
    - Added SAM4S4 for completeness ([`ef0930b`](https://github.com/kiibohd/BOSSA/commit/ef0930bc3f4b2907a2e590961d969c5908c89add))
    - Fixed indentation so it matches ([`ef643a6`](https://github.com/kiibohd/BOSSA/commit/ef643a6ca58056ed565d341bd8026c3b7ba1a638))
    - Added support for SAM4S2 processors. Modified Makefile so it will compile despite deprecated usleep() ([`8ff8542`](https://github.com/kiibohd/BOSSA/commit/8ff8542e322b0ef8205fbc1e86c290ad34909d35))
    - Merge branch 'serial_fix' of https://github.com/8devices/BOSSA into arduino ([`d98eebb`](https://github.com/kiibohd/BOSSA/commit/d98eebbb1c5cd6bc799958f09f58a4ce905e985e))
    - Merge branch 'openbsd' of https://github.com/dj3vande/BOSSA into arduino ([`f0f40de`](https://github.com/kiibohd/BOSSA/commit/f0f40de67b9e4e13a8195a7b75224c63dcd9f151))
    - Close the serial port before we try to open it again ([`dde0439`](https://github.com/kiibohd/BOSSA/commit/dde0439a080a5ca70255a53e9a81f7d1563015a2))
    - Don't divide by zero on small transfers ([`e0aefbd`](https://github.com/kiibohd/BOSSA/commit/e0aefbd0e9c3021a63ff9f5c3ee9c11f804cf1e3))
    - Raw serial I/O requires clearing IEXTEN ([`5c9bcf3`](https://github.com/kiibohd/BOSSA/commit/5c9bcf305ef314214937af40b6113827a7eb4dec))
    - Add OpenBSD support ([`a9ec1e8`](https://github.com/kiibohd/BOSSA/commit/a9ec1e8f354867cd293a5c306116a6227ddd2896))
    - Fixed reading from SAMD devices ([`8f4ba72`](https://github.com/kiibohd/BOSSA/commit/8f4ba72aaf7dbe12861ea0503b17c03f3dcdabb7))
    - Merge pull request #12 from femtoio/arduino ([`63cdfbb`](https://github.com/kiibohd/BOSSA/commit/63cdfbb6592e254cbbf74be95554bad0beb2dee7))
    - Merge branch 'arduino' of github.com:shumatech/BOSSA into arduino ([`a90a0ca`](https://github.com/kiibohd/BOSSA/commit/a90a0ca610b4722239c0cd27e24099ea78d4e56e))
    - The SAM R21 is being bootloaded with the USB CDC only version of the SAM-BA bootloader for SAM D21, since the SAM R21 is essentially a SAM D21 + built-in AT86RF233 ([`1a63f56`](https://github.com/kiibohd/BOSSA/commit/1a63f5631c1e1a3e53c17d5b9da54d3957f46082))
    - Adding support for the SAM R21 (ATSAMR21E18A) chip, updating support for reset on the ATSAMD21E18A ([`ed74ab3`](https://github.com/kiibohd/BOSSA/commit/ed74ab3e211ffd320fb151d15a7f80f109e0537a))
    - Added fast verify command support ([`2920b32`](https://github.com/kiibohd/BOSSA/commit/2920b32899c5db4473b62db9fa2b14a9c224e4b4))
    - Added Samba::crc16AddByte function ([`fc29665`](https://github.com/kiibohd/BOSSA/commit/fc29665ed21e4a931446a248ba60d14b82c5063b))
    - Fixed strange error when programming SAMD21 from UART ([`ed06a70`](https://github.com/kiibohd/BOSSA/commit/ed06a7027f19c14690481b7f2e43416343f99954))
    - Removed variable 'offset' where unused ([`3fc979a`](https://github.com/kiibohd/BOSSA/commit/3fc979aa397719c0bd13596d040a043415511a81))
    - Fixed bug when uploading large binaries (>64K) ([`0b761b1`](https://github.com/kiibohd/BOSSA/commit/0b761b1b961c773996f17fd0651f39b6dfe325b1))
    - Check for partial writes in Samba::writeBinary() ([`b380a58`](https://github.com/kiibohd/BOSSA/commit/b380a58182f477c920329e3dc589309b68f9d303))
    - Use a constant to define block size in Samba::write(..) ([`d83d094`](https://github.com/kiibohd/BOSSA/commit/d83d094c3e387f66d906c56c671dfe5ac3f53dda))
    - Added support for Arduino extended multi-page write command ([`59e57b5`](https://github.com/kiibohd/BOSSA/commit/59e57b55079ba4821d72d385cc87949fadcf3140))
    - Added support for Arduino extended chip-erase command ([`e3f4009`](https://github.com/kiibohd/BOSSA/commit/e3f4009614af4f0401f66e53c711c76efcbb3021))
    - Merge pull request #9 from femtoio/arduino ([`053e0f5`](https://github.com/kiibohd/BOSSA/commit/053e0f5891d5ef60f186b0628bc041b780189fc2))
    - Merge branch 'arduino'. Pulling in support for some SAM D21 chips, including the ATSAMD21E18A. ([`c207200`](https://github.com/kiibohd/BOSSA/commit/c207200e44dfbf3d2a23772ab4bf1c57da003d21))
    - Adding support for the ATSAMD21E18A chip ([`31757e0`](https://github.com/kiibohd/BOSSA/commit/31757e013d69dfb9b32b1812a138fdc3982847ed))
    - More precise operation timing (with ms precision) ([`2468c40`](https://github.com/kiibohd/BOSSA/commit/2468c40ab61bb66a1947701fbfdb8bebd6377a2c))
    - Fixed wrong 'size' parameter in Flash::loadBuffer ([`66903b6`](https://github.com/kiibohd/BOSSA/commit/66903b674eb7173aa50d798ebb821b2e7bd6a142))
    - Fixed formula for calculating beginning of the Flash ([`3e6bf7a`](https://github.com/kiibohd/BOSSA/commit/3e6bf7ab6a4ab9a67701251c50fa73d3a7584fe6))
    - Removed some useless field by inlining into formula ([`e9b8d0b`](https://github.com/kiibohd/BOSSA/commit/e9b8d0b588cb4f1d00b624acc4b0eec306019da1))
    - Increased lenght of version string. ([`05a99d6`](https://github.com/kiibohd/BOSSA/commit/05a99d680666dd1332e84f5471b0e811a2072ec2))
    - Improved flash write on SAMD21 ([`78373f1`](https://github.com/kiibohd/BOSSA/commit/78373f1d853d512da18e7db10b2629f91116d31f))
    - Removed unused loadBuffer method and field ([`a48bb7e`](https://github.com/kiibohd/BOSSA/commit/a48bb7eb74965e6a1bf6afb9a86defc17a1fa511))
    - Removed unused namespace import ([`4926bcc`](https://github.com/kiibohd/BOSSA/commit/4926bcceaeaec2dbd7421bc25b0b8a8c7d1a6d11))
    - Added device descriptors metadata for SAMD21xx chips ([`24f1328`](https://github.com/kiibohd/BOSSA/commit/24f132889da449c04f032513c53fca06ead84646))
    - Removed unused _eraseAuto field ([`1a54221`](https://github.com/kiibohd/BOSSA/commit/1a54221b2a8ffbf9b6ba9ccd2db43a8685c22441))
    - Fixed help message for erase command (SAMDxx specific) ([`1e249f9`](https://github.com/kiibohd/BOSSA/commit/1e249f990f383189dcca7ee81f78b944f5bdc559))
    - Print time taken to complete various operations ([`d19fa83`](https://github.com/kiibohd/BOSSA/commit/d19fa830073111ba69eec19a9014794568302277))
    - Removed useless NmvFlash::appStartPage() method ([`9a4c7b7`](https://github.com/kiibohd/BOSSA/commit/9a4c7b790e73a52c5ce0b019f82a561f050f0e1c))
    - Removed useless PAGE_SIZE_IN_BYTES macro ([`c8f26c2`](https://github.com/kiibohd/BOSSA/commit/c8f26c2dabae3dcf9be6d4d30290043684db9b76))
    - Cosmetic make up. No code change. ([`3c1c5f1`](https://github.com/kiibohd/BOSSA/commit/3c1c5f1d4715391ce1b7be183a045ad66563ca2f))
    - Restore termios settings after using serial port ([`832d3b7`](https://github.com/kiibohd/BOSSA/commit/832d3b7f57acbef7fd58328ad2168efaf9f09def))
    - Merge pull request #6 from 8devices/arduino ([`3e35adb`](https://github.com/kiibohd/BOSSA/commit/3e35adb9fb4db2e2df5e1dfe7e40904e1a124214))
    - Fix system crash when using bossac on UART ([`fd0cb87`](https://github.com/kiibohd/BOSSA/commit/fd0cb87d15e320afc3577445286ee215b8077102))
    - Added support for (new) Atmel SAMD21 XPlained board ([`2473c8a`](https://github.com/kiibohd/BOSSA/commit/2473c8afb2c24c74c0c11f12ddaeb75a37e6ffaf))
    - Removed always-true checks (unsiged vs signed) ([`01bac24`](https://github.com/kiibohd/BOSSA/commit/01bac249785cf43abc62e708e18d252ccf072d44))
    - Use of portable stdlib.h include for malloc function. ([`3250e19`](https://github.com/kiibohd/BOSSA/commit/3250e19c6ef65f021b05997fe7507bd52423af0e))
    - Indentation pass to match the rest of the project style ([`601161f`](https://github.com/kiibohd/BOSSA/commit/601161ff6df0497a97dcecf36a46de346d496e06))
    - Workaround for SAM3 sam-ba bug on chip identification. ([`6a42cb6`](https://github.com/kiibohd/BOSSA/commit/6a42cb6d173f1999062ad30e40f9ebccd431a414))
    - Removed ChipInfo structure ([`ed8802c`](https://github.com/kiibohd/BOSSA/commit/ed8802c66455a542acc6bd5ff48b4b2df8f13ad8))
    - Slightly reworded informative message ([`82fa4c3`](https://github.com/kiibohd/BOSSA/commit/82fa4c35f3fcd5049e54544c447fe6509be6085f))
    - Using memmove instead of memcpy to remove dependency on GLIBC_2.14 ([`b10d3f7`](https://github.com/kiibohd/BOSSA/commit/b10d3f70b7cf2c7a32e202ce1ce3c471abf84190))
    - Allow 100ms delay after chip reset to avoid some linux kernel lock-ups. ([`5620800`](https://github.com/kiibohd/BOSSA/commit/562080098f99764e4365bc044b90b1a456baa7a7))
    - Removed useless beforeWrite() API. ([`0012a5c`](https://github.com/kiibohd/BOSSA/commit/0012a5c48d4e00693cc092e4158b48bf320d500a))
    - Add support for page offestting when issuing flash commands. The callee will not have any knowledge on this. The caller will use the information provided by the flash abstraction to decide on offsetting page numbers (if needed). This is mainly done to support cortex m0+ based SAM D21 devices, in which both the bootloader and the application reside in flash. ChangeLog: Define an API that returns the application's start page. Use that API in places where a write/verify on flash is performed.Read operation is an exemption. Remove hardcoding of page offsetting in writePage/readPage methods in Flash implementations ([`204c82a`](https://github.com/kiibohd/BOSSA/commit/204c82a09a8c6015b2f32773bb25de552f5d29d7))
    - And finally glow the LED :). The problem is with shifting the address before writing to page cache. Now we write to the page buffer , and shift the address when writing to the address register ([`8e8f1eb`](https://github.com/kiibohd/BOSSA/commit/8e8f1eba4b88b871d2023a8a3c193717bcd119dd))
    - Refactor code for readability ([`a39f2b6`](https://github.com/kiibohd/BOSSA/commit/a39f2b653ad3d42e991f6e3f3c1b710b3289a96c))
    - Fix build error. My bad ([`1b18170`](https://github.com/kiibohd/BOSSA/commit/1b181702814c2f51fed332ddba217e471bf32d7d))
    - Implement reset functionality for CORTEX M0+ controllers ([`040f2f2`](https://github.com/kiibohd/BOSSA/commit/040f2f2a2b5fd4f54e6e23429ecf34ebaa2fbd43))
    - Fix invalid NVM Module base address for SAMG18 ([`8d648a5`](https://github.com/kiibohd/BOSSA/commit/8d648a570935be0fbdec4da24126613dbab76269))
    - Do more code cleanup Fix a bug with page write that will cause some data to be lost Make internal helper fumctions private ([`f9dc54f`](https://github.com/kiibohd/BOSSA/commit/f9dc54f847271755065032603943f99cac1a4b89))
    - Implement lock region properly, by properly calculating the addressing scheme ([`37e73ad`](https://github.com/kiibohd/BOSSA/commit/37e73adc112f2dad95310844c852de820c55ebc1))
    - Merge branch 'master' of https://github.com/shumatech/BOSSA into arduino ([`c3467c9`](https://github.com/kiibohd/BOSSA/commit/c3467c905f71be945ba194984b6934e363bab150))
    - Implement page write and partial page write. Add callback that will be called before page write on each flash controller implementation Add bufferSize as payload to the loadBuffer callback. This is needed to implement partial page writes ([`b750e12`](https://github.com/kiibohd/BOSSA/commit/b750e129665a27db1177b03a97efb12bfb246cec))
    - Fix issues with addressing scheme, when chip erase. Fix issues with writePage with partial pages. Partial write need more fixes. ([`d788611`](https://github.com/kiibohd/BOSSA/commit/d788611dabba582acc2ddc7f1f451db754a46aa4))
    - Implement and verify flash read operation ([`6542090`](https://github.com/kiibohd/BOSSA/commit/65420901108e37dff0804b42379a0a7d12ffff56))
    - Implement set_bor (brown out reset) ([`901e75a`](https://github.com/kiibohd/BOSSA/commit/901e75a8335690c10061a396049319f836b103d2))
    - Implement Set_Security_Bit ([`7e8e62b`](https://github.com/kiibohd/BOSSA/commit/7e8e62bc83bbff86400ce4483c42973740570c9c))
    - Implement Bod enable/disable. The value will be  preserved till next reset. ([`8d8c2b2`](https://github.com/kiibohd/BOSSA/commit/8d8c2b28da09c959680a64bc06185312cfc2938d))
    - Implement erase flash. Implement wait() before a command is issued to the nvm controller ([`2c6bb93`](https://github.com/kiibohd/BOSSA/commit/2c6bb93a3aa7ed9951ff0221a82459b2f8381407))
    - Fix comments from review (i.e Modify M0 to M0_plus) ([`bb03463`](https://github.com/kiibohd/BOSSA/commit/bb03463a4c6e8f917a0a547c0c3c0d5b4851b55f))
    - Add support for read page and more ([`b99e199`](https://github.com/kiibohd/BOSSA/commit/b99e19980f5eda356eb24430476e1897e0910ec8))
    - Verify Lock/unlock, set security bit on D21J18 ([`42b79ae`](https://github.com/kiibohd/BOSSA/commit/42b79ae01f6c5af08666c6c09374b06db9a6d689))
    - Add initial support for SAM D21 NVM controller ([`b4137b2`](https://github.com/kiibohd/BOSSA/commit/b4137b2135a420371377ee78098cf2515ea35bad))
    - Added missing files for previous commmit. ([`df1147e`](https://github.com/kiibohd/BOSSA/commit/df1147ee90d3f9cc58fd3ab4fdb1f345d8ac207e))
    - Added priliminary support for Cortex M0+ devices ([`bc71af4`](https://github.com/kiibohd/BOSSA/commit/bc71af4e009b2419b583e70b5427109785b3d4f4))
    - Adjusted auto-reset delay (500ms is enough) ([`fdfb6ff`](https://github.com/kiibohd/BOSSA/commit/fdfb6fffb2787f140e15a3e3e647ae93e70c24b2))
    - Arduino Auto-restart tests ([`f6f90ce`](https://github.com/kiibohd/BOSSA/commit/f6f90ceee286b0ccc75d2212bbec7985f3450ed3))
    - Removed ARM disassembler code that is released under the GPL ([`af5b9ba`](https://github.com/kiibohd/BOSSA/commit/af5b9baa70de169021458eeea397d3dbd42a1568))
    - Updated license information in source code comment blocks ([`7601b72`](https://github.com/kiibohd/BOSSA/commit/7601b722b5046e370707be2c3027177df3af0aa1))
    - Workaround for upload error on macosx (not sure what is the cause of this) ([`71a90ac`](https://github.com/kiibohd/BOSSA/commit/71a90ac90a374e5229f26d0b223e55e4dd71d8dc))
    - Added missing include ([`28c7fc4`](https://github.com/kiibohd/BOSSA/commit/28c7fc46428705609ccd038d3bb2078ecf482a05))
    - Added command line option to override USB port autodetection ([`8489f66`](https://github.com/kiibohd/BOSSA/commit/8489f66031e4fc3be8a35b9ca52f997b8692596f))
    - Added uart speed param to Samba::connect(..) ([`0b73b98`](https://github.com/kiibohd/BOSSA/commit/0b73b98bc48f5dc6d918c81452eaee6b54a1a6a0))
    - Workaround for linux CDC-ACM driver bug ([`8dcb47d`](https://github.com/kiibohd/BOSSA/commit/8dcb47d562a98287a8d349bccc98cad284624ed0))
    - Give some time after setting boot flash to switch banks ([`ab54c85`](https://github.com/kiibohd/BOSSA/commit/ab54c859c635eb87b241b9ef7bab759d78ce6a5f))
    - Added 'reset' command. ([`76f975f`](https://github.com/kiibohd/BOSSA/commit/76f975f949960e505f68c9684d20428d6d9c9862))
    - Fixed assert in EefcFlash. Fixed USB buffering (for macosx but improved CDC drivers will probably suffer the issue) ([`4c14377`](https://github.com/kiibohd/BOSSA/commit/4c143772202c2beed6a7eb67b69b2531112b1663))
    - Corrected help message in bossash ([`05bfcc3`](https://github.com/kiibohd/BOSSA/commit/05bfcc39bc0453c3028b1161175b95a81af7a901))
    - Added SAM4S support Fixed problem with SAM9XE GPNVM bits Match SAM7SE32 user address to datasheet ([`5f9b830`](https://github.com/kiibohd/BOSSA/commit/5f9b8305d00d5837404d82b38854d7d85c1fb865))
    - Added SAM3A support ([`8a34164`](https://github.com/kiibohd/BOSSA/commit/8a341643df4ba2a2b2f0e7e383dff64618777f83))
    - Added SAM3X support (Cristian Maglie) ([`d680d2e`](https://github.com/kiibohd/BOSSA/commit/d680d2edccc09fda5dab12ad55d89dc1737f3e33))
    - String internationalization patch (Cristian Maglie) ([`2bd8522`](https://github.com/kiibohd/BOSSA/commit/2bd85228a68086a9a4b1d48e3e3b001bbb321f12))
    - Mingw doesn't like %z so a cast it is ([`8f9ad07`](https://github.com/kiibohd/BOSSA/commit/8f9ad076bca5a6b724db015fcdc078dd3d58a005))
    - Corrected printf format for ssize_t ([`f35437b`](https://github.com/kiibohd/BOSSA/commit/f35437b891113b08daac4fc2928f8c4603fbdf76))
    - Update copyrights ([`64a82a9`](https://github.com/kiibohd/BOSSA/commit/64a82a96f6f387455d388b8d80d05db96e8e34d1))
    - Added chip ID's for newer revision SAM7S256 and SAM7S128 ([`c71e9ac`](https://github.com/kiibohd/BOSSA/commit/c71e9ac9546f845920e2244a5be1028bb887b2f8))
    - Added more PIO operations to the shell. ([`8ee2cd6`](https://github.com/kiibohd/BOSSA/commit/8ee2cd6d5ddb4c58a712d53f8821283822787f05))
    - Completed flash GPNVM bit commands for bossa shell. ([`dd79065`](https://github.com/kiibohd/BOSSA/commit/dd7906566ede0c57f00ca4e65a52f18c189f3e05))
    - Changed dmg background and script to include bossash Don't compress executables under Mac OS X due to upx instability Changed the way history is displayed to accommodate MAC OS X's older readline library ([`afccbda`](https://github.com/kiibohd/BOSSA/commit/afccbda6bd227bd31a13636c4db4b0349527506a))
    - Removed trailing whitespace ([`e69d520`](https://github.com/kiibohd/BOSSA/commit/e69d520c023173dd6815fabf591fd518afc16362))
    - Minor porting issues on Linux ([`885493b`](https://github.com/kiibohd/BOSSA/commit/885493bf91ecead7a45c5851b3ab33c5ea4f78b3))
    - New BOSSA shell command line application to do basic memory, flash, and PIO diagnostics Workaround for SAM3U firmware bug Fixed a bug with setting boot to flash bit on SAM3 devices ([`acea002`](https://github.com/kiibohd/BOSSA/commit/acea00237e48acd5f91549b3d4092267a3bccacd))
    - Added a workaround for SAM3 firmware bug where reads from flash return all zeros by copying the flash to SRAM and reading it from there. Fixed a bug in erase all for dual plane EEFC devices. ([`c0e29fa`](https://github.com/kiibohd/BOSSA/commit/c0e29fa9f3f2402282831c07c87f33803fb599a4))
    - Added missing headers ([`8304cab`](https://github.com/kiibohd/BOSSA/commit/8304cabba981461dba80a1c04d04381919850e34))
    - Changed chip ID detection logic Changed Windows installer version Ported USB serial port changes to Windows ([`7e26bd6`](https://github.com/kiibohd/BOSSA/commit/7e26bd683d664c3ac8c66cd50c32b07cbc25309d))
    - Changed Linux port factor to support usb flag. ([`a50a0b6`](https://github.com/kiibohd/BOSSA/commit/a50a0b6b9cab99c5e6adbec7c12435a9baf977f7))
    - Ported to MAC OS-X Added flush to address a SAM firmware bug ([`14db622`](https://github.com/kiibohd/BOSSA/commit/14db622723260af4b368960d872c122e9cda1ec0))
    - Delete unused png's. Adjusted bossa form for more room in write options. ([`c747f33`](https://github.com/kiibohd/BOSSA/commit/c747f333497a1b00da5a60c675a8d2eb512a2539))
    - Increase main window width. ([`3b88625`](https://github.com/kiibohd/BOSSA/commit/3b886256e4d7fec77c1eb5e61f03732230f9c842))
    - Fixed version padding when getting samba info string. ([`379a242`](https://github.com/kiibohd/BOSSA/commit/379a24222f55c1aeb2d5de4711e26b405ca93072))
    - Changed to use BMP bitmaps instead of PNG to avoid libpng conflicts.  Fixed problem with setting text in info dialog. Handle cancel when read, write, or verify thread is running.  Detect ttyACM devices as USB.  Changed SAM-BA timout after auto-detect to one second. ([`104e599`](https://github.com/kiibohd/BOSSA/commit/104e599d88a44247846a95305971e8f1a297f7ed))
    - Changed cancel button on progress window to a standard button sizer. ([`ec81a17`](https://github.com/kiibohd/BOSSA/commit/ec81a17801be808cd3cd8af51087cdf10c6b7763))
    - Added cancel button to progress window.  Adjusted size of top fields in info window. ([`5e40e55`](https://github.com/kiibohd/BOSSA/commit/5e40e55ee20ccbf06b7796935f4895f698cb33e1))
    - Added Wix Windows installer.  Made a few minor debug output tweaks. ([`a26296a`](https://github.com/kiibohd/BOSSA/commit/a26296a2b69035716812be730602ee9066f16f80))
    - Merge branch 'master' of scott-arch:bossa ([`e37836f`](https://github.com/kiibohd/BOSSA/commit/e37836fcd570f457a9b49ddef4ea65ce98a11042))
    - Made wxWidgets version reference header define Cleaned up cr/lf endings ([`9884d0c`](https://github.com/kiibohd/BOSSA/commit/9884d0c557e75fa6d516812c3526f9019985181f))
    - Changed the posix serial port termios flags ([`3e7e8b5`](https://github.com/kiibohd/BOSSA/commit/3e7e8b51825c8770ce847ed06193aa511862b597))
    - Initial porting to Linux ([`40f102b`](https://github.com/kiibohd/BOSSA/commit/40f102bc298df356ba0e60a702639ece345b5fc6))
    - Initial commit ([`30f2385`](https://github.com/kiibohd/BOSSA/commit/30f2385ea8b098874a6cd91f5facec1580502e34))
</details>

