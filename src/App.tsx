import * as React from 'react'
import * as wasm from './wasm/replay_tools'
import { AppShell, Button, FileButton, Group, Header, Navbar, NavLink, Title } from '@mantine/core'

const ReplayContext = React.createContext(null);

function MyNavbar() {
  return (
    <Navbar width={{ base: 200 }} p="xs">
      <NavLink
        label="Replay Heatmap"
        active
      />
    </Navbar>
  )
}

function OpenReplay() {
  const resetRef = React.useRef<() => void>(null);
  const { replay, setReplay } = React.useContext(ReplayContext)
  const clearFile = () => {
    setReplay(null);
    wasm.clear()
    resetRef.current?.();
  };
  return (
    <Group position="apart" align="center">
      <FileButton onChange={setReplay} accept=".wotreplay">
        {(props) => <Button {...props}>Open Replay</Button>}
      </FileButton>
      <Button disabled={!replay} color="red" onClick={clearFile}>Reset</Button>
    </Group>
  )

}
function MyHeader({ children }) {
  return (
    <Header height={60} p="md">
      <Group position="apart" align="center">
        <Title order={3}>Wot Replay Tools</Title>
        {children}
      </Group>
    </Header>
  )
}
function App() {
  const [replay, setReplay] = React.useState(null)

  return (
    <ReplayContext.Provider value={{ replay, setReplay }}>
      <AppShell
        padding="md"
        navbar={<MyNavbar />}
        header={<MyHeader children={<OpenReplay />} />}
        styles={(theme) => ({
          main: { backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[8] : theme.colors.gray[0] },
        })}
      >
        <Canvas />
      </AppShell>
    </ReplayContext.Provider>

  )
}

function Canvas() {
  const canvasRef = React.useRef(null)
  const contextRef = React.useRef(null)
  const { replay, setReplay } = React.useContext(ReplayContext)
  const [map, setMap] = React.useState(null)
  React.useEffect(() => {
    if (replay) {
      replay.arrayBuffer().then(bytes => {
        const res = wasm.parse_replay(new Uint8Array(bytes))
        const map = JSON.parse(wasm.get_map())
        setMap(map)
      })
    }

  }, [replay])
  
  React.useEffect(() => {
    if (canvasRef && map) {
      const canvas = canvasRef.current
        canvas.width = map.width
        canvas.height = map.height

        const ctx = canvas.getContext("2d")
        const img = ctx.getImageData(0, 0, canvas.height, canvas.width)
        const pixels = img.data
        const positons = JSON.parse(wasm.positions())

        for (const pos of positons) {
          let { avatar_id, x, y } = pos
          x = Math.trunc(x)
          y = Math.trunc(y)
          const offset = (y * img.width + x) * 4;

          const r = 255
          const g = 0
          const b = 0
          pixels[offset] = r;
          pixels[offset + 1] = g;
          pixels[offset + 2] = b;
          pixels[offset + 3] = 255;
        }
        ctx.putImageData(img, 0, 0)
    }
    
  }, [canvasRef, map])
  if(!map) return <div>Open a Replay</div>
  return ( 
    <div className='relative'>
      <img height={map.height} width={map.width} className="absolute" src={`src/assets/maps/${map.id}.png`} alt="" />
      <canvas height={map.height} width={map.width}  className='absolute top-0 left-0 z-20' ref={canvasRef} />
    </div>
  )
}


export default App
