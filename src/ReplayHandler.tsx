import { Button, FileButton, Group } from "@mantine/core";
import * as React from "react";
import * as wasm from "./wasm/replay_tools";

export const ReplayContext = React.createContext<IReplayContext>({
    replay: null,
    setReplay: () => {},
});

export default function ReplayHandler() {
    const resetRef = React.useRef<() => void>(null);
    const { replay, setReplay } = React.useContext(ReplayContext);

    const clearFile = () => {
        setReplay(null);
        wasm.clear();
        resetRef.current?.();
    };

    return (
        <Group position="apart" align="center">
            <FileButton onChange={setReplay} accept=".wotreplay">
                {(props) => <Button {...props}>Open Replay</Button>}
            </FileButton>
            <Button disabled={!replay} color="red" onClick={clearFile}>
                Reset
            </Button>
        </Group>
    );
}

type IReplayContext = {
    replay: File | null;
    setReplay: (value: File | null) => void;
};
