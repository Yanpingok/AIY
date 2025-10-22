// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import '@mysten/dapp-kit/dist/index.css';
import './index.css';
import '@fontsource-variable/inter';
import '@fontsource-variable/red-hat-mono';

import { AiyClientProvider, WalletProvider } from '@mysten/dapp-kit';
import { getFullnodeUrl } from '@mysten/aiy/client';
import { QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { queryClient } from './lib/queryClient';
import { router } from './routes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<QueryClientProvider client={queryClient}>
			<AiyClientProvider
				defaultNetwork="aiy:mainnet"
				networks={{
					'aiy:testnet': { url: getFullnodeUrl('testnet') },
					'aiy:mainnet': { url: getFullnodeUrl('mainnet') },
					'aiy:devnet': { url: getFullnodeUrl('devnet') },
				}}
			>
				<WalletProvider>
					<RouterProvider router={router} />
				</WalletProvider>
			</AiyClientProvider>
		</QueryClientProvider>
	</React.StrictMode>,
);
