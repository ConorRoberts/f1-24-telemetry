import { createLazyFileRoute } from "@tanstack/react-router";
import { useEffect, useMemo, useRef, useState } from "react";
import {
  LineChart,
  Line,
  ResponsiveContainer,
  CartesianGrid,
  Legend,
} from "recharts";
import { config } from "../config";
import type { components } from "~/openapi";

const defaultTelemetryEvent: components["schemas"]["CarTelemetryEvent"] = {
  brake: 0,
  throttle: 0,
  speed: 0,
  type: "car_telemetry",
};

const defaultMotionEvent: components["schemas"]["CarMotionEvent"] = {
  g_force_lateral: 0,
  g_force_longitudinal: 0,
  g_force_vertical: 0,
  type: "car_motion",
  world_position_x: 0,
  world_position_y: 0,
  world_position_z: 0,
};

const maxArraySize = 500;

export const Route = createLazyFileRoute("/")({
  component: () => {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    const [carData, setCarData] = useState<
      components["schemas"]["CarTelemetryEvent"][]
    >(
      [...Array.from({ length: maxArraySize })].map(
        () => defaultTelemetryEvent,
      ),
    );

    const [motionData, setMotionData] = useState<
      components["schemas"]["CarMotionEvent"][]
    >([...Array.from({ length: maxArraySize })].map(() => defaultMotionEvent));

    const motionArr = useRef<typeof motionData>([]);

    useEffect(() => {
      const es = new EventSource(`${config.apiHost}/events`, {
        withCredentials: false,
      });

      es.onmessage = (e) => {
        try {
          const json: components["schemas"]["Event"] = JSON.parse(e.data);

          if (json.type === "car_telemetry") {
            setCarData((prev) => [
              ...(prev.length >= maxArraySize ? prev.slice(1) : prev),
              json,
            ]);
          } else if (json.type === "car_motion") {
            setMotionData((prev) => [
              ...(prev.length >= maxArraySize ? prev.slice(1) : prev),
              json,
            ]);

            motionArr.current = [
              ...(motionArr.current.length >= maxArraySize
                ? motionArr.current.slice(1)
                : motionArr.current),
              json,
            ];

            if (canvasRef.current) {
              // TODO: Show car moving around track
            }
          }
        } catch (_e) {
          return;
        }
      };

      es.onerror = (e) => {
        console.error(e);
      };
    }, []);

    const _averageSpeed = useMemo(
      () => carData.reduce((acc, e) => acc + e.speed, 0) / carData.length,
      [carData],
    );

    const latestSpeed = useMemo(() => carData.at(-1)?.speed ?? 0, [carData]);
    const latestMotion = useMemo(() => motionData.at(-1), [motionData]);

    return (
      <div className="p-4 flex flex-col gap-4">
        <div className="flex gap-4">
          <div
            style={{ height: "300px", width: "300px" }}
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
            style={{ height: "300px", width: "300px" }}
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
          </div>
        </div>
        <div className="bg-gray-100 rounded-lg p-4 font-semibold">
          <p>{latestSpeed} km/h</p>
        </div>
        {latestMotion && (
          <div className="flex items-center gap-1">
            <div className="bg-gray-100 rounded-lg p-4 font-semibold flex items-center">
              <p>X: {latestMotion.g_force_lateral}</p>
            </div>
            <div className="bg-gray-100 rounded-lg p-4 font-semibold flex items-center">
              <p>Y: {latestMotion.g_force_vertical}</p>
            </div>
            <div className="bg-gray-100 rounded-lg p-4 font-semibold flex items-center">
              <p>Z: {latestMotion.g_force_longitudinal}</p>
            </div>
          </div>
        )}

        <canvas
          className="bg-gray-100 border border-dashed"
          // style={{
          //   width: `${canvasWidth}px`,
          //   height: `${canvasHeight}px`,
          // }}
          ref={canvasRef}
        />
      </div>
    );
  },
});
