import React, { useState } from 'react';
import GenerateInsightsButton from '../common/GenerateInsightsButton';
import UsernameInput from '../inputs/UsernameInput';
import GamesCountInput from '../inputs/GamesCountInput';
import GameModeInput from '../inputs/GameModeInput';
import UserColorInput from '../inputs/UserColorInput';

const DefaultPlaceholder = {
    username: 'DrNykterstein', // Magnus Carlsen
    gamesCount: 20,            // Number of games to fetch
    gameMode: 'blitz',         // Blitz is most popular mode
    userColor: 'both'   
};

export interface InputProps {
    username: string,
    gamesCount: number,
    gameMode: string,
    userColor: string
}

interface InputPanelProps {
    handleSendData: (props: InputProps) => void;
    usernameNotFound: boolean;
}

export const InputsPanel: React.FC<InputPanelProps> = ({ handleSendData, usernameNotFound }) => {
    const [username, setUsername] = useState<string>(DefaultPlaceholder.username);
    const [gamesCount, setGamesCount] = useState<number>(DefaultPlaceholder.gamesCount);
    const [gameMode, setGameMode] = useState<string>(DefaultPlaceholder.gameMode);
    const [userColor, setUserColor] = useState<string>(DefaultPlaceholder.userColor);

    const handleClick = () => {
        handleSendData({
          username,
          gamesCount,
          gameMode,
          userColor
        });
      };

    return (
        <div className="bg-gray-800 p-7 rounded-lg shadow-md mb-8">
            <UsernameInput value={username} onChange={setUsername} userNameNotFound={usernameNotFound}/>
            <GamesCountInput  value={gamesCount} onChange={setGamesCount} max={50} /> 
            <GameModeInput value={gameMode} onChange={setGameMode} />
            <UserColorInput value={userColor} onChange={setUserColor} />
            <GenerateInsightsButton onClick={handleClick} />
        </div>
    );
};