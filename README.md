# Arwing

Arwing's goal is to be a querying engine for finding interactions in a set of SSBM slippi replays.

## Usage
### Arwing-cli
 ```
    ./arwing -p PLAYER_CHARACTER -o OPPONENT_CHARACTER -d DIRECTORY_TO_PARSE -i INTERACTION -e JSON_EXPORT_LOCATION
 ```
* PLAYER_CHARACTER and OPPONENT_CHARACTER are all-caps character names with space delimited by underscores, ex: GAME_AND_WATCH

* DIRECTORY_TO_PARSE parses recursively through each folder and finds each .slp file to analyze. Symlinks are ignored.

* INTERACTION is a set of three data fields repeated any amount of times: 
    * Character Performing Action (In the same format as PLAYER_CHARACTER)
    * ACTION_STATE of the action ([Action State IDs](https://docs.google.com/spreadsheets/d/1JX2w-r2fuvWuNgGb6D3Cs4wHQKLFegZe2jhbBuIhCG8/preview#gid=13 "Action State IDs"), [Character-Specific Action State IDs](https://docs.google.com/spreadsheets/d/1Nu3hSc1U6apOhU4JIJaWRC4Lj0S1inN8BFsq3Y8cFjI/preview "Action State IDs"))
    * Amount of frames between previous action and this action

* EXPORT: If the -e flag is specified a json file will be exported for use in in-engine replay with Clippi (loaded in the same way as any clippi combo.json)



## It aims to answer questions like:

- What do top foxes do after drilling pikachu?
- How do players DI out of chaingrabs?
- What does a particular opponent like to do out of laser?

## In the future it might point out when a player gets punished for:

- Missing L-cancels
- Not getting enough galint on ledgedash
- Getting stuck in shine
- Not extending combos as far as possible
- And much much more!

### TODO
- Parse data stream instead of in-memory
- Evaluate using concurrency/parallelism
- Evaluate using arrow
- Cache Data
- Refine query system
    - Select stages
    - Parse by Player Name
    - Add Meta-Actions i.e. From-ledge, Wavedash, Retreat, Advance

- Create GUI App
    - Likely Tauri
    - Represent user flow