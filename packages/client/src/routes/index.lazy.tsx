import { createLazyFileRoute } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import { LineChart, Line, ResponsiveContainer } from "recharts";
import { config } from "../config";

export const Route = createLazyFileRoute("/")({
  component: () => {
    const [data, setData] = useState<Data[]>([]);

    useEffect(() => {
      const es = new EventSource(`${config.apiHost}/events`, {
        withCredentials: false,
      });

      es.onmessage = (e) => {
        const d = getData(e.data);

        if (d) {
          setData((prev) => [
            ...(prev.length >= 500 ? prev.slice(1) : prev),
            d,
          ]);
        }
      };

      es.onerror = (e) => {
        console.error(e);
      };
    }, []);

    return (
      <div style={{ height: "600px", width: "500px" }} className="bg-black">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart data={data}>
            <Line
              dot={false}
              type="monotone"
              dataKey="throttle"
              stroke="green"
              strokeWidth={2}
            />
            <Line
              dot={false}
              type="monotone"
              dataKey="brake"
              stroke="red"
              strokeWidth={2}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    );
  },
});

type Data = {
  type: "data";
  throttle: number;
  brake: number;
};

const getData = (s: string) => {
  try {
    return JSON.parse(s) as Data;
  } catch (_e) {
    return null;
  }
};
