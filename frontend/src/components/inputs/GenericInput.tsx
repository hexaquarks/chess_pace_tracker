import React from 'react';

interface GenericInputProps<T> {
  label: string;
  type: 'text' | 'number' | 'select';
  value: T;
  onChange: (value: T) => void;
  options?: { value: string | number; label: string }[];
  max?: number;
}

const GenericInput = <T extends number | string>({ label, type, value, onChange, options, max }: GenericInputProps<T>) => (
  <div>
    <label className="block text-white text-sm font-bold mb-2" htmlFor={label}>
      {label}
    </label>
    {type === 'select' ? (
      <select
        className="appearance-auto bg-gray-800 border-none w-full text-white mr-3 py-1 px-2 -ml-1 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
        id={label}
        value={value}
        onChange={(e) => onChange(e.target.value as T)}
      >
        {options?.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
    ) : (
      <input
        className="appearance-none bg-transparent border-none w-full text-white mr-3 py-1 px-2 leading-tight focus:outline-none"
        id={label}
        type={type}
        placeholder={`Enter ${label.toLowerCase()}`}
        value={value}
        onChange={(e) => onChange(type === 'number' ? (e.target.valueAsNumber as T) : (e.target.value as T))}
        max={max}
      />
    )}
    <div className="border-b border-gray-500 mb-2"></div>
  </div>
);

export default GenericInput;