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
    <div className="flex flex-col items-center justify-between w-screen bg-zinc-900 pt-20 pb-8 min-h-screen">
      <div className="flex flex-row justify-center w-full max-w-7xl">
        <div className="flex flex-col w-full max-w-2xl px-4">
          <div className="bg-gray-800 p-7 rounded-lg shadow-md mb-8">
            <UsernameInput value={username} onChange={setUsername} />
            <GamesCountInput value={gamesCount} onChange={setGamesCount} max={50} />
            <GameModeInput value={gameMode} onChange={setGameMode} />
            <UserColorInput value={userColor} onChange={setUserColor} />
            <SendDataButton onClick={handleSendData} />
          </div>

          {response && (
            <div className="bg-gray-800 p-6 rounded-lg shadow-md">
              <ResponsePanel
                time={response.time}
                explanationMessage={response.explanation_message}
              />
            </div>
          )}
        </div>

        {response && (
          <div className="w-1/4 max-h-[calc(100vh*0.45)] overflow-y-auto self-begin">
            <ErrorsPanel
              gamesWithError={response.games_with_errors}
            />
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
