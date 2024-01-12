import React, { useState } from 'react';
import './App.css';

interface RequestInformation {
  username: string;
  games_count: number;
}

function App() {
  const [username, setUsername] = useState<string>('');
  const [gamesCount, setGamesCount] = useState<number>(0);

  const sendDataToBackend = async () => {
    try {
      const payload: RequestInformation = {
        username: username,
        games_count: gamesCount
      };
      console.log('Sending:', payload);

      const response = await fetch('http://localhost:8000/fetch-chess-data', {
        method: 'POST',
        body: JSON.stringify(payload),
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();
      console.log(data);
    } catch (error) {
      console.error('Error sending data to backend', error);
    }
  };

  const handleGamesCountChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = parseInt(event.target.value, 10);
    setGamesCount(isNaN(value) ? 0 : value);
  };

  return (
    <div className="App">
      <header className="App-header">
        <input
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          placeholder="Enter username"
        />
        <input
          type="number"
          value={gamesCount}
          onChange={handleGamesCountChange}
          placeholder="Enter number of games"
        />
        <button onClick={sendDataToBackend}>Send Data</button>
      </header>
    </div>
  );
}

export default App
