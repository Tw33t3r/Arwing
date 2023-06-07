import characterMoves, { Move } from "./moves";

export type Character = {
  name: string;
  internalId: number;
  icon: string;
  moves: Move[];
}

export enum InternalCharacters {
  MARIO = 0,
  FOX = 1,
  CAPTAIN_FALCON = 2,
  DONKEY_KONG = 3,
  KIRBY = 4,
  BOWSER = 5,
  LINK = 6,
  SHEIK = 7,
  NESS = 8,
  PEACH = 9,
  POPO = 10,
  NANA = 11,
  PIKACHU = 12,
  SAMUS = 13,
  YOSHI = 14,
  JIGGLYPUFF = 15,
  MEWTWO = 16,
  LUIGI = 17,
  MARTH = 18,
  ZELDA = 19,
  YOUNG_LINK = 20,
  DR_MARIO = 21,
  FALCO = 22,
  PICHU = 23,
  GAME_AND_WATCH = 24,
  GANONDORF = 25,
  ROY = 26,
  MASTER_HAND = 27,
  CRAZY_HAND = 28,
  WIRE_FRAME_MALE = 29,
  WIRE_FRAME_FEMALE = 30,
  GIGA_BOWSER = 31,
  SANDBAG = 32,
};

export const characters: Character[] = Object.keys(InternalCharacters).filter(key => (typeof InternalCharacters[key as any as number] === 'number')).map(key => ({ name: key, internalId: parseInt(InternalCharacters[key as any as number]), icon: "src", moves: characterMoves(key)}));
