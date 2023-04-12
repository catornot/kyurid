# kyurid

a plugin that gives the sqvm access to a high level api for audio

# DOCS REAL?

any errors produced by io or rodio will raise errors that you will have to catch with `try` and `catch`

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

## KYGetSound
```c
array<string> function KYGetSounds(string mod_name)
```
gets all the sounds from a folder

# rodio sink api
for now only basic stuff ( I am too laszy to add more )

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

## KYDestroySink
```c
void function KYDestroySink( string id )
```
destroys the sink which makes also makes the id invalid
