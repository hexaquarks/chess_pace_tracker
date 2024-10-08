import React from 'react';

interface GameModeInputProps {
    value: string;
    onChange: (value: string) => void;
}

const GameModeInput: React.FC<GameModeInputProps> = ({ value, onChange }) => (
    <div>
        <label
            className="block text-white text-xs xs:text-sm font-bold mb-2"
            htmlFor="game-mode"
        >
            Game mode
        </label>
        <select
            className="appearance-none pl-3 bg-gray-800 text-sm xs:text-base border-none w-full text-white mr-3 py-1 px-2 -ml-1 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
            id="game-mode"
            value={value}
            onChange={(e) => onChange(e.target.value)}
        >
            <option value="bullet">Bullet</option>
            <option value="blitz">Blitz</option>
            <option value="rapid">Rapid</option>
        </select>
        <div className="border-b border-gray-500 mb-2"></div> {/*This is for the bottom bar*/}
    </div>
);

export default GameModeInput;
