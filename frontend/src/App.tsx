import React, { useState } from 'react';
import UsernameInput from './components/UsernameInput';
import GamesCountInput from './components/GamesCountInput';
import GameModeInput from './components/GameModeInput';
import UserColorInput from './components/UserColorInput';
import SendDataButton from './components/SendDataButton';
import { sendDataToBackend, ResponseInformation, MessageInformationAssessment } from './services/apiService';
import ResponsePanel from './components/ResponsePanel';
import ErrorsPanel from './components/ErrorsPanel';

function App() {
  const [username, setUsername] = useState<string>('physicskush');
  const [gamesCount, setGamesCount] = useState<number>(3);
  const [gameMode, setGameMode] = useState<string>('blitz');
  const [userColor, setUserColor] = useState<string>('white');

  const [response, setResponse] = useState<ResponseInformation | null>(null);

  const handleSendData = async () => {
    const responseData: ResponseInformation = await sendDataToBackend(username, gamesCount, gameMode, userColor);
    setResponse(responseData);
  };

  return (
    <div className="flex felx-row items-center justify-center w-screen bg-zinc-900">
      <div className="flex flex-col items-center justify-center h-screen">
        <div className="mb-8 w-full max-w-2xl px-4">
          <div className="bg-gray-800 p-6 rounded-lg shadow-md w-full">
            <div className="mb-4">
              <UsernameInput value={username} onChange={setUsername} />
            </div>
            <div className="mb-4">
              <GamesCountInput value={gamesCount} onChange={setGamesCount} max={50} />
            </div>
            <div className="mb-4">
              <GameModeInput value={gameMode} onChange={setGameMode} />
            </div>
            <div className="mb-4">
              <UserColorInput value={userColor} onChange={setUserColor} />
            </div>
            <div className="flex items-center justify-between">
              <SendDataButton onClick={handleSendData} />
            </div>
          </div>
        </div>

        {response && (
          <div className="w-full max-w-2xl px-4">
            <ResponsePanel
              time={response.time}
              explanationMessage={response.explanation_message}
            />
          </div>
        )}
      </div>
      {response && (
        <div className="w-1/4 flex flex-col items-center justify-center h-screen">
          <ErrorsPanel
            gamesWithError={response.games_with_errors}
          />
        </div>
      )}

    </div>

  );
}

export default App;
