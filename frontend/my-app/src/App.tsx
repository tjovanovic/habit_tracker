import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.css';

function App() {

  const [state, setState] = useState("El blanco vido...")

  useEffect(() => {
    const url = "/api";

    const fetchData = async () => {
      try {
        const response = await fetch(url, { method: "POST" });
        const json = await response.text();
        console.log(json);
        setState(json);
      } catch (error) {
        console.log("error", error);
      }
    };

    fetchData();
  }, []);

  return (
    <div className="App">
      Test -- {state}
    </div>
  );
}

export default App;
