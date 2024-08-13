import React from 'react';

interface UserColorInputProps {
    value: string;
    onChange: (value: string) => void;
}

const UserColorInput: React.FC<UserColorInputProps> = ({ value, onChange }) => (
    <div>
        <label
            className="block text-white text-xs xs:text-sm font-bold mb-2"
            htmlFor="user-color"
        >
            User color
        </label>
        <select
            className="appearance-none pl-3 bg-gray-800 text-sm xs:text-base border-none w-full text-white mr-3 py-1 px-2 -ml-1 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
            id="user-color"
            value={value}
            onChange={(e) => onChange(e.target.value)}
        >
            <option value="both">Both</option>
            <option value="white">White</option>
            <option value="black">Black</option>
        </select>
        <div className="border-b border-gray-500 mb-2"></div> {/*For bottom bar*/}
    </div>
);

export default UserColorInput;
