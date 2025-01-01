import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import React from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';


type Data = {
  type:"data",
  throttle:number;
  brake:number;
}

const getData = (s:string)=>{
  try{
      return JSON.parse(s) as Data
  }catch(e){
    return null;
  }
}

function App() {
  const [count, setCount] = useState(0);
  const [data,setData] = useState<Data[]>([])

  useEffect(()=>{
    const es = new EventSource("http://172.19.177.92:4000/events",{withCredentials:false});

    es.onmessage =(e)=>{
      const d = getData(e.data);

      if (d){

        setData(prev=>[...(prev.length>=500 ? prev.slice(1) : prev),d])
      }
    }

    es.onerror = (e)=>{
      console.error(e)
    }
  },[])

  return (
    <div style={{height:"600px",width:"500px"}}>
     <ResponsiveContainer width="100%" height="100%">
      <LineChart data={data}>
        <Line type="monotone" dataKey="throttle" stroke="green" strokeWidth={2} />
        <Line type="monotone" dataKey="brake" stroke="red" strokeWidth={2} />
      </LineChart>
    </ResponsiveContainer>
    </div>
  )
}

export default App
