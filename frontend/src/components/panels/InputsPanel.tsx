import React, { useState } from 'react';
import SendDataButton from '../common/SendDataButton';
import UsernameInput from '../inputs/UsernameInput';
import GamesCountInput from '../inputs/GamesCountInput';
import GameModeInput from '../inputs/GameModeInput';
import UserColorInput from '../inputs/UserColorInput';

export interface InputProps {
    username: string,
    gamesCount: number,
    gameMode: string,
    userColor: string
}

interface InputPanelProps {
    handleSendData: (props: InputProps) => Promise<void>;
    usernameNotFound: boolean;
}

export const InputsPanel: React.FC<InputPanelProps> = ({ handleSendData, usernameNotFound }) => {
    const [username, setUsername] = useState<string>('physicskush');
    const [gamesCount, setGamesCount] = useState<number>(3);
    const [gameMode, setGameMode] = useState<string>('blitz');
    const [userColor, setUserColor] = useState<string>('white');

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
            <SendDataButton onClick={handleClick} />
        </div>
    );
};