import { type FC, useEffect, useMemo, useRef, useState } from "react";
import { LineChart, Line, ResponsiveContainer, CartesianGrid } from "recharts";
import { config } from "../config";
import type { components } from "../openapi";

const defaultTelemetryEvent: components["schemas"]["CarTelemetryEvent"] = {
  brake: 0,
  throttle: 0,
  speed: 0,
  type: "car_telemetry",
  brake_temp: [],
  tyre_inner_temp: [],
  tyre_surface_temp: [],
  engine_temperature: 0,
  tyre_pressure: [],
  timestamp: 0,
};

const defaultMotionEvent: components["schemas"]["CarMotionEvent"] = {
  g_force_lateral: 0,
  g_force_longitudinal: 0,
  g_force_vertical: 0,
  type: "car_motion",
  world_position_x: 0,
  world_position_y: 0,
  world_position_z: 0,
  timestamp: 0,
};

const maxArraySize = 1000;
const pointSize = 15;

const Tires: FC<{ data: components["schemas"]["CarTelemetryEvent"] }> = (
  props,
) => {
  return (
    <div className="grid grid-cols-2 gap-1 w-max h-max">
      {[...Array.from({ length: 4 })].map((_, i) => (
        <div
          key={i.toString()}
          className="size-16 bg-gray-100 rounded-full flex items-center justify-center"
        >
          <p className="font-extrabold">{props.data.tyre_surface_temp[i]}</p>
        </div>
      ))}
    </div>
  );
};

export const App = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const [carData, setCarData] = useState<
    components["schemas"]["CarTelemetryEvent"][]
  >([...Array.from({ length: maxArraySize })].map(() => defaultTelemetryEvent));

  const [motionData, setMotionData] = useState<
    components["schemas"]["CarMotionEvent"][]
  >([...Array.from({ length: maxArraySize })].map(() => defaultMotionEvent));

  // Mapping timestamp to data. Need to insert into this object by selecting a timestamp close enough to the incoming data's timestamp
  const lapData = useRef<
    Record<string, { x: number; y: number; speed: number }>
  >({});

  useEffect(() => {
    const connect = async () => {
      const ctl = new AbortController();

      const response = await fetch(`${config.apiHost}/events`, {
        headers: {
          Accept: "text/event-stream",
        },
        signal: ctl.signal,
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      if (!response.body) {
        throw new Error("ReadableStream not supported");
      }

      const reader = response.body.getReader();
      const decoder = new TextDecoder();
      let buffer = "";

      while (true) {
        const { value, done } = await reader.read();

        if (done) break;

        buffer += decoder.decode(value, { stream: true });

        const events = buffer.split("\n\n");
        buffer = events.pop() ?? "";

        for (const e of events) {
          if (e.trim().length === 0) {
            continue;
          }
          const json: components["schemas"]["Event"] = JSON.parse(e.slice(5));
          if (json.type === "car_motion") {
            lapData.current[Math.floor(json.timestamp * 1_000_000)] = {
              x: json.world_position_x,
              y: json.world_position_z,
              speed: 0,
            };

            const canvas = canvasRef.current;
            if (!canvas) {
              return;
            }
            const ctx = canvas.getContext("2d");
            if (!ctx) {
              return;
            }
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            const l = Object.values(lapData.current);
            const scale = 2;
            const xPoints = l.map((e) => e.x * scale);
            const yPoints = l.map((e) => e.y * scale);
            const bounds = {
              minX: Math.min(...xPoints),
              maxX: Math.max(...xPoints),
              minY: Math.min(...yPoints),
              maxY: Math.max(...yPoints),
            };
            canvas.width = bounds.maxX * 2;
            canvas.height = bounds.maxY * 2;
            ctx.translate(canvas.width / 2, canvas.height / 2);
            ctx.beginPath();
            ctx.strokeStyle = `hsl(${Math.floor(Math.random() * 360)} 100% 50%)`;
            ctx.lineWidth = pointSize;
            ctx.lineCap = "round";
            ctx.lineJoin = "round";

            let i = 0;
            for (const { x, y } of l) {
              if (i === 0) {
                ctx.moveTo(x, y);
              } else {
                ctx.lineTo(x, y);
                ctx.stroke();
              }
              i++;
            }
          }
        }
      }

      // const es = new EventSource(`${config.apiHost}/events`, {
      //   withCredentials: false,
      // });

      // const motionDataIdx = 0;
      // let telemetryDataIdx = 0;
      // let currentLap = 1;
      // let lastEventTime = Date.now();

      // es.onmessage = (e) => {
      //   // console.log(`Delay: ${Date.now() - lastEventTime}`);
      //   lastEventTime = Date.now();
      //   try {
      //     const json: components["schemas"]["Event"] = JSON.parse(e.data);

      //     if (json.type === "car_telemetry") {
      //       if (telemetryDataIdx % 2 === 0) {
      //         // setCarData((prev) => [
      //         //   ...(prev.length >= maxArraySize ? prev.slice(1) : prev),
      //         //   json,
      //         // ]);
      //       }

      //       telemetryDataIdx++;
      //     } else if (json.type === "lap_data") {
      //       if (json.current_lap_num !== currentLap) {
      //         currentLap = json.current_lap_num;

      //         // Show map
      //       }
      //     } else if (json.type === "car_motion") {
      //       // setMotionData((prev) => [
      //       //   ...(prev.length >= maxArraySize ? prev.slice(1) : prev),
      //       //   json,
      //       // ]);
      //       // lapData.current[Math.floor(json.timestamp * 1_000_000)] = {
      //       //   x: json.world_position_x,
      //       //   y: json.world_position_z,
      //       //   speed: 0,
      //       // };
      //       // const canvas = canvasRef.current;
      //       // if (!canvas) {
      //       //   return;
      //       // }
      //       // const ctx = canvas.getContext("2d");
      //       // if (!ctx) {
      //       //   return;
      //       // }
      //       // ctx.clearRect(0, 0, canvas.width, canvas.height);
      //       // const l = Object.values(lapData.current);
      //       // const scale = 2;
      //       // const xPoints = l.map((e) => e.x * scale);
      //       // const yPoints = l.map((e) => e.y * scale);
      //       // const bounds = {
      //       //   minX: Math.min(...xPoints),
      //       //   maxX: Math.max(...xPoints),
      //       //   minY: Math.min(...yPoints),
      //       //   maxY: Math.max(...yPoints),
      //       // };
      //       // canvas.width = bounds.maxX * 2;
      //       // canvas.height = bounds.maxY * 2;
      //       // ctx.translate(canvas.width / 2, canvas.height / 2);
      //       // ctx.beginPath();
      //       // ctx.strokeStyle = `hsl(${Math.floor(Math.random() * 360)} 100% 50%)`;
      //       // ctx.lineWidth = pointSize;
      //       // ctx.lineCap = "round";
      //       // ctx.lineJoin = "round";
      //       // let i = 0;
      //       // for (const { x, y } of l) {
      //       //   if (i === 0) {
      //       //     ctx.moveTo(x, y);
      //       //   } else {
      //       //     ctx.lineTo(x, y);
      //       //     ctx.stroke();
      //       //   }
      //       //   i++;
      //       // }
      //       // motionDataIdx++;
      //     }
      //   } catch (e) {
      //     console.error(e);
      //   }
      // };

      // es.onerror = (e) => {
      //   console.error(e);
      // };
    };

    connect();
  }, []);

  const _averageSpeed = useMemo(
    () => carData.reduce((acc, e) => acc + e.speed, 0) / carData.length,
    [carData],
  );

  const latestSpeed = useMemo(() => carData.at(-1)?.speed ?? 0, [carData]);
  const latestCarData = useMemo(() => carData.at(-1), [carData]);
  const _latestMotion = useMemo(() => motionData.at(-1), [motionData]);

  return (
    <div className="p-1 flex flex-col gap-4">
      <div className="flex gap-4">
        <div
          style={{ height: "300px", width: "600px" }}
          className="bg-white border border-gray-200"
        >
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={carData}>
              <CartesianGrid />
              <Line
                dot={false}
                type="monotone"
                dataKey="throttle"
                stroke="#00ff04"
                strokeWidth={2}
              />

              <Line
                dot={false}
                type="monotone"
                dataKey="brake"
                stroke="#eb4034"
                strokeWidth={2}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
        <div
          style={{ height: "300px", width: "600px" }}
          className="bg-white border border-gray-200"
        >
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={carData}>
              <CartesianGrid />
              <Line
                dot={false}
                type="monotone"
                dataKey="speed"
                stroke="#0044ff"
                strokeWidth={2}
              />
            </LineChart>
          </ResponsiveContainer>
        </div>
        {/* <div
          style={{ height: "300px", width: "600px" }}
          className="bg-white border border-gray-200"
        >
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={motionData}>
              <CartesianGrid />
              <Legend />
              <Line
                dot={false}
                type="monotone"
                dataKey="g_force_lateral"
                stroke="#03c4aa"
                strokeWidth={2}
              />
              <Line
                dot={false}
                type="monotone"
                dataKey="g_force_vertical"
                stroke="#da7400"
                strokeWidth={2}
              />
              <Line
                dot={false}
                type="monotone"
                dataKey="g_force_longitudinal"
                stroke="#ad01b3"
                strokeWidth={2}
              />
            </LineChart>
          </ResponsiveContainer>
        </div> */}
        <div className="bg-gray-100 border p-4 font-semibold h-max">
          <p>{latestSpeed} km/h</p>
        </div>
        {latestCarData && <Tires data={latestCarData} />}
      </div>

      <canvas
        className="bg-gray-100 border w-[1200px] h-[600px]"
        ref={canvasRef}
      />
    </div>
  );
};

const _TrackMap = () => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  return (
    <canvas
      className="bg-gray-100 border w-[1200px] h-[600px]"
      ref={canvasRef}
    />
  );
};
