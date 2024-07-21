import React from 'react';

interface UsernameInputProps {
	value: string;
	onChange: (value: string) => void;
	userNameNotFound: boolean;
}

const UsernameInput: React.FC<UsernameInputProps> = ({ value, onChange, userNameNotFound }) => (
	<div>
		<label className="block text-white text-sm font-bold mb-2" htmlFor="username">
			User name
		</label>
		<input
			className={`appearance-none bg-transparent border ${userNameNotFound ? 'border-red-500' : 'border-none'
				} w-full text-white mr-3 py-1 px-2 leading-tight focus:outline-none`}
			id="username"
			type="text"
			placeholder="Enter username"
			value={value}
			onChange={(e) => onChange(e.target.value)}
		/>
		<div className="border-b border-gray-500 mb-2"></div>
	</div>
);

export default UsernameInput;
