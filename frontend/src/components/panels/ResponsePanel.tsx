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
  const iconSize = 'h-12 sm:h-20 w-12 sm:w-20';

  const IconGetter = () => {
    switch (assessment) {
      case 0: return <CheckCircleIcon className={`${iconStyles[0]} ${iconSize}`} />;
      case 1: return <ExclamationTriangleIcon className={`${iconStyles[1]} ${iconSize}`} />;
      case 2: return <ExclamationCircleIcon className={`${iconStyles[2]} ${iconSize}`} />;
      default: return null;
    }
  };

  return (
    <div className="bg-gray-800 p-5 items-center rounded-lg flex space-x-3 text-white w-full">
      <div className="">
        <IconGetter />
      </div>
      <div className="flex-col">
        <p className="text-sm xs:text-md sm:text-lg font-semibold pb-2">
          {message}
        </p>
        <p className="text-sm xs:text-sm sm:text-lg">
          {`Time: ${time}s`}
        </p>
      </div>
    </div>
  );
  
};

export default ResponsePanel;