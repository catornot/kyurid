# kyurid

a plugin that gives the sqvm access to a high level api for audio

# DOCS REAL?

any errors produced by io or rodio will raise errors that you will have to catch with `try` and `catch`

to **fetch** sounds this plugin starts from the titanfall 2 folder and looks at 

`/R2Northstar/mods` + mod name + `/sounds` + sound name

this means that currenlty profiles aren't supported and if the mod's folder name changes it wouldn't work too

the mod name issue can probably be solved by using `NSGetCurrentModName` or not I am not sure how it works.

# util funcs

## KYPlaySoundFile
```c
void function KYPlaySoundFile( string mod_name, string sound_name )
```
low overhead

just spawns a sound and plays it until it ends or when the game stops runnning

good for short sounds

## KYPlaySoundFileTracked
```c
void function KYPlaySoundFileTracked( string mod_name, string sound_name )
```
higher overhead, can also fill up the memory since sinks will only be cleaned on client destruction

## KYGetSounds
```c
array<string> function KYGetSounds(string mod_name)
```
gets all the sounds from a folder

# rodio sink api
for now only basic stuff ( I am too lazy to add more )

every sink is destroyed at the end of a `Client` VM even tho you can call the functions on `UI` or `CLIENT` VMs

## KYCreateSoundSink
```c
string function KYCreateSoundSink()
```
creates a rodio sink and returns a id for it

## KYAddSoundToSink
```c
void function KYAddSoundToSink( string id, string mod_name, string sound_name )
```
appends a sound to the sink

## KYSkipSinkSound
```c
void function KYSkipSinkSound( string id )
```
skips the currently playing sound

## KYSinkSetSpeed
```c
void function KYSinkSetSpeed( string id, float speed )
```
speeds up the sounds >:3

normal speed is 1.0

## KYSinkSetVolume
```c
void function KYSinkSetVolume( string id, float volume )
```
changes the volume

normal volume is 1.0

## KYSinkGetVolume
```c
`float function KYSinkGetVolume( string id )
```
returns the current volume of the sink

## KYSinkGetSpeed
```c
float function KYSinkGetSpeed( string id )
```
returns the current speed of the sink

## KYSinkSetPause
```c
void function KYSinkSetPause( string id, bool paused )
```
attempts to pause or play the current sound in the sink

does nothing if it's already in that state

## KYSinkGetPause
```c
bool function KYSinkGetPause( string id )
```
returns the pause state of the sink

## KYDestroySink
```c
void function KYDestroySink( string id )
```
destroys the sink which makes also makes the id invalid
