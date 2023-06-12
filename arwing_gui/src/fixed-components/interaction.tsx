import { Accessor, Component} from "solid-js";

import InteractionRow from "./interaction-row";

type PlayerOpponentProps = {
  playerId: Accessor<number>,
  opponentId: Accessor<number>,
}

//TODO(Tweet): Double check to make sure that I want to pass props twice instead of using context
const Interactions: Component<PlayerOpponentProps> = (props) => {

  return (
    <div>
      <h2 class="mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 md:text-4xl lg:text-5xl">
        Interaction
      </h2>
      <InteractionRow
        playerId={props.playerId}
        opponentId={props.opponentId}
        setInteraction={(characterId: number, moveId: number, withinFrames: number) => console.log("TODO") }
        />
    </div>
  );
}

export default Interactions;
