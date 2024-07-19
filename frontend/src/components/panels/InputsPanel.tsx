import React, { useState } from 'react';
import SendDataButton from '../common/SendDataButton';
import GenericInput from '../inputs/GenericInput'

export interface InputProps {
    username: string,
    gamesCount: number,
    gameMode: string,
    userColor: string
}

interface InputPanelProps {
    handleSendData: (props: InputProps) => Promise<void>;
}

export const InputsPanel: React.FC<InputPanelProps> = ({ handleSendData }) => {
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
            <GenericInput
                label="User name"
                type="text"
                value={username}
                onChange={setUsername}
            />
            <GenericInput
                label="Games count"
                type="number"
                value={gamesCount}
                onChange={setGamesCount}
                max={50}
            />
            <GenericInput
                label="Game mode"
                type="select"
                value={gameMode}
                onChange={setGameMode}
                options={[
                    { value: 'bullet', label: 'Bullet' },
                    { value: 'blitz', label: 'Blitz' },
                    { value: 'rapid', label: 'Rapid' },
                ]}
            />
            <GenericInput
                label="User color"
                type="select"
                value={userColor}
                onChange={setUserColor}
                options={[
                    { value: 'both', label: 'Both' },
                    { value: 'white', label: 'White' },
                    { value: 'black', label: 'Black' },
                ]}
            />
            <SendDataButton onClick={handleClick} />
        </div>
    );
};