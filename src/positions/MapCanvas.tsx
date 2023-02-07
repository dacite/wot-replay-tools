import * as React from 'react'
import { ReplayContext } from '../ReplayHandler'
import * as wasm from "../wasm/replay_tools"

export default function MapCanvas() {
  const canvasRef = React.useRef<HTMLCanvasElement | null>(null)
  const { replay, setReplay } = React.useContext(ReplayContext)
  const [map, setMap] = React.useState<Map | null>(null)

  React.useEffect(() => {
    if (replay) {
      replay.arrayBuffer().then(bytes => {
        wasm.parse_replay(new Uint8Array(bytes))

        const map = JSON.parse(wasm.get_map())
        setMap(map)
      })
    } else {
      if (canvasRef && canvasRef.current) clearCanvas(canvasRef.current)
      setMap(null)
    }

  }, [replay])

  React.useEffect(() => {
    if (!canvasRef || !map) return;
    const canvas = canvasRef.current!
    canvas.width = map.width
    canvas.height = map.height
    const ctx = canvas.getContext("2d")

    const imgData = ctx?.getImageData(0, 0, canvas.height, canvas.width)!
    const positions: MapPosition[] = JSON.parse(wasm.positions())
    const modifiedImgData = drawPositions(imgData, positions, [255, 0, 0, 255])

    ctx?.putImageData(modifiedImgData, 0, 0)

  }, [canvasRef, map])

  if (!map) return <div>Open a Replay</div>

  return (
    <div className='relative'>
      <img height={map.height} width={map.width} className="absolute" src={`src/assets/maps/${map.id}.png`} alt="" />
      <canvas height={map.height} width={map.width} className='absolute top-0 left-0 z-20' ref={canvasRef} />
    </div>
  )
}

interface Map {
  id: number
  height: number
  width: number
}

interface MapPosition {
  x: number,
  y: number,
  z: number,
  avatar_id: number
}

function drawPositions(imgArray: ImageData, positions: MapPosition[], color: number[]) {
  for (let { x, y } of positions) {
    x = Math.trunc(x)
    y = Math.trunc(y)

    const offset = (y * imgArray.width + x) * 4
    imgArray.data[offset] = color[0]
    imgArray.data[offset + 1] = color[1]
    imgArray.data[offset + 2] = color[2]
    imgArray.data[offset + 3] = color[3]
  }

  return imgArray
}

function clearCanvas(canvas: HTMLCanvasElement) {
  const ctx = canvas.getContext("2d")
  
  ctx?.clearRect(0, 0, canvas.width, canvas.height)
}