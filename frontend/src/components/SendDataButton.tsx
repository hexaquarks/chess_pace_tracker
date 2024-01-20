// components/SendDataButton.tsx
import React from 'react';

interface SendDataButtonProps {
  onClick: () => void;
}

const SendDataButton: React.FC<SendDataButtonProps> = ({ onClick }) => (
  <button
    className="bg-blue-500 hover:bg-blue-700 text-white font-bold mt-2 -mb-2 py-2 px-4 rounded focus:outline-none focus:shadow-outline"
    type="button"
    onClick={onClick}
  >
    Send Data
  </button>
);

export default SendDataButton;
