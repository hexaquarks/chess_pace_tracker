import React from 'react';
import { ExclamationCircleIcon, ExclamationTriangleIcon, CheckCircleIcon } from '@heroicons/react/24/outline';

interface ResponsePanelProps {
  time: number;
  explanationMessage: [string, number];
}

const iconStyles = [
  'text-green-500', // Success
  'text-yellow-500', // Warning
  'text-red-500', // Error
];

const ResponsePanel: React.FC<ResponsePanelProps> = ({ time, explanationMessage }) => {

  const [message, assessment] = explanationMessage;
  const iconSize = 'h-20 w-20';

  const IconGetter = () => {
    switch (assessment) {
      case 0: return <CheckCircleIcon className={`${iconStyles[0]} ${iconSize}`} />;
      case 1: return <ExclamationTriangleIcon className={`${iconStyles[1]} ${iconSize}`} />;
      case 2: return <ExclamationCircleIcon className={`${iconStyles[2]} ${iconSize}`} />;
      default: return null;
    }
  };

  return (
    <div className="flex items-center space-x-2 rounded-lg bg-gray-800 text-white w-full">
      <div className="flex-shrink-0">
        <IconGetter />
      </div>
      <div className="flex-grow">
        <p className="text-lg font-semibold">{message}</p>
        <p>{`Time: ${time}s`}</p>
      </div>
    </div>
  );
};

export default ResponsePanel;