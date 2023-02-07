import * as React from "react";
import { AppShell } from "@mantine/core";
import ReplayHandler, { ReplayContext } from "./ReplayHandler";
import Navbar from "./Navbar";
import Header from "./Header";
import MapCanvas from "./positions/MapCanvas";

function App() {
    const [replay, setReplay] = React.useState<File | null>(null);

    return (
        <ReplayContext.Provider value={{ replay, setReplay }}>
            <AppShell
                padding="md"
                navbar={<Navbar />}
                header={
                    <Header>
                        <ReplayHandler />
                    </Header>
                }
                styles={(theme) => ({ main: { backgroundColor: theme.colors.dark[8] } })}
            >
                <MapCanvas />
            </AppShell>
        </ReplayContext.Provider>
    );
}

export default App;
