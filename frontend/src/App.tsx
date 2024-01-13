import React, { useState } from 'react';
import UsernameInput from './components/UsernameInput';
import GamesCountInput from './components/GamesCountInput';
import GameModeInput from './components/GameModeInput';
import UserColorInput from './components/UserColorInput';
import SendDataButton from './components/SendDataButton';
import { sendDataToBackend } from './services/apiService';

function App() {
  const [username, setUsername] = useState<string>('');
  const [gamesCount, setGamesCount] = useState<number>(0);
  const [gameMode, setGameMode] = useState<string>('bullet');
  const [userColor, setUserColor] = useState<string>('white');

  return (
    <div className="flex justify-center items-center h-screen bg-gray-900">
      <div className="bg-gray-800 p-6 rounded-lg shadow-md w-full max-w-md">
        <div className="mb-4">
          <UsernameInput value={username} onChange={setUsername} />
        </div>
        <div className="mb-4">
          <GamesCountInput value={gamesCount} onChange={setGamesCount} max={50}/>
        </div>
        <div className="mb-4">
          <GameModeInput value={gameMode} onChange={setGameMode} />
        </div>
        <div className="mb-4">
          <UserColorInput value={userColor} onChange={setUserColor} />
        </div>
        <div className="flex items-center justify-between">
          <SendDataButton onClick={() => sendDataToBackend( username, gamesCount, gameMode, userColor )} />
        </div>
      </div>
    </div>
  );
}

export default App;
