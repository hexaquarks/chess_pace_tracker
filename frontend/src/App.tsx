import React, { useState } from 'react';
import './App.css';

function App() {
  const [username, setUsername] = useState('');
  const [gamesCount, setGamesCount] = useState('');

  const sendDataToBackend = async () => {
    try {
      const payload = { username, gamesCount };
    console.log('Sending payload:', payload);
      const response = await fetch('http://localhost:8000/fetch-chess-data', {
        method: 'POST',
        body: JSON.stringify({ 
          username: username, 
          games_count: gamesCount 
        }),
        headers: {
          'Content-Type': 'application/json',
        }
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();
      console.log(data); // Handle the response data as needed
    } catch (error) {
      console.error('Error sending data to backend', error);
    }
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
          type="text"
          value={gamesCount}
          onChange={(e) => setGamesCount(e.target.value)}
          placeholder="Enter number of games"
        />
        <button onClick={sendDataToBackend}>Send Data</button>
      </header>
    </div>
  );
}

export default App;
