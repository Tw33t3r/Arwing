import characterMoves, { Move } from "./moves";

export type Character = {
  name: string;
  internalId: number;
  icon: string;
  moves: Move[];
}

export enum ExternalCharacters {
  CaptainFalcon = 0,
  DonkeyKong = 1,
  Fox = 2,
  GameAndWatch = 3,
  Kirby = 4,
  Bowser = 5,
  Link = 6,
  Luigi = 7,
  Mario = 8,
  Marth = 9,
  Mewtwo = 10,
  Ness = 11,
  Peach = 12,
  Pikachu = 13,
  IceClimbers = 14,
  Jigglypuff = 15,
  Samus = 16,
  Yoshi = 17,
  Zelda = 18,
  Sheik = 19,
  Falco = 20,
  YoungLink = 21,
  DrMario = 22,
  Roy = 23,
  Pichu = 24,
  Ganondorf = 25,
};

export const characters: Character[] = Object.keys(ExternalCharacters).filter(key => (typeof ExternalCharacters[key as any as number] === 'number')).map(key => ({ name: key, internalId: parseInt(ExternalCharacters[key as any as number]), icon: "src", moves: characterMoves(key) }));
