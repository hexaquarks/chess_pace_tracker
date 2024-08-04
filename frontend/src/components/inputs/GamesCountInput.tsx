import React from 'react';

interface GamesCountInputProps {
  value: number;
  onChange: (value: number) => void;
  max: number;
}

const GamesCountInput: React.FC<GamesCountInputProps> = ({ value, onChange, max }) => (
  <div>
    <label className="block text-white text-xs xs:text-sm font-bold mb-2" htmlFor="games-count">
      Games count
    </label>
    <input
      className="appearance-none bg-transparent text-sm xs:text-base border-none w-full text-white mr-3 py-1 px-2 leading-tight focus:outline-none"
      id="games-count"
      type="number"
      placeholder="0"
      value={value}
      onChange={(e) => onChange(e.target.valueAsNumber)}
      max={max}
    />
    <div className="border-b border-gray-500 mb-2"></div>
  </div>
);

export default GamesCountInput;
