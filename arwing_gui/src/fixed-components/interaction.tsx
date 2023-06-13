import { Accessor, Component, For } from "solid-js";
import { createStore } from "solid-js/store";

import InteractionRow from "./interaction-row";
import { Character } from "../consts";

type PlayerOpponentProps = {
  player: Accessor<Character>,
  opponent: Accessor<Character>,
  ref: any
}

export type InteractionData = {
  interactionId: number,
  characterId: number,
  moveId: number,
  withinFrames: number,
}

const Interactions: Component<PlayerOpponentProps> = (props) => {
  let interactionId = 0;

  const [interactionData, setInteractionData] = createStore<InteractionData[]>([{ interactionId: 0, characterId: props.player().internalId, moveId: props.player().moves[0].moveId as any as number, withinFrames: 1000 }]);

  const setCharacterId = (characterId: number, interactionId: number) => {
    setInteractionData(interaction => interaction.interactionId === interactionId, "characterId", characterId);
  }
  const setMoveId = (moveId: number, interactionId: number) => {
    setInteractionData(interaction => interaction.interactionId === interactionId, "moveId", moveId);
  }
  const setWithinFrames = (withinFrames: number, interactionId: number) => {
    setInteractionData(interaction => interaction.interactionId === interactionId, "withinFrames", withinFrames);
  }
  const newInteraction = () => {
    setInteractionData([...interactionData, { interactionId: ++interactionId, characterId: props.player().internalId, moveId: props.player().moves[0].moveId as any as number, withinFrames: 1000 }]);
  }

  props.ref(interactionData);

  return (
    <div>
      <h2 class="mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 md:text-4xl lg:text-5xl">
        Interaction
      </h2>
      <For each={interactionData}>
        {(interaction) => {
          return <div>
            <InteractionRow
              player={props.player}
              opponent={props.opponent}
              setCharacterId={setCharacterId}
              setMoveId={setMoveId}
              setWithinFrames={setWithinFrames}
              interactionId={interaction.interactionId}
            />
          </div>
        }}
      </For>
      <button
        type="button"
        onClick={() => newInteraction()}
        class="text-white bg-gradient-to-br from-purple-600 to-blue-500 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-2 mb-2"
      >

        New Interaction
      </button>
    </div>
  );
}

export default Interactions;
