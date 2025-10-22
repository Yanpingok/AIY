// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const concepts = [
	'concepts',
	'concepts/components',
	{
		type: 'category',
		label: 'App Developers',
		link: {
			type: 'doc',
			id: 'concepts/app-devs',
		},
		items: [
			{
				type: 'category',
				label: 'Object Model',
				link: {
					type: 'doc',
					id: 'concepts/object-model',
				},
				items: [
					{
						type: 'category',
						label: 'Object Ownership',
						link: {
							type: 'doc',
							id: 'concepts/object-ownership',
						},
						items: [
							'concepts/object-ownership/address-owned',
							'concepts/object-ownership/immutable',
							'concepts/object-ownership/party',
							'concepts/object-ownership/shared',
							'concepts/object-ownership/wrapped',
						],
					},
					{
						type: 'category',
						label: 'Transfers',
						link: {
							type: 'doc',
							id: 'concepts/transfers',
						},
						items: ['concepts/transfers/custom-rules', 
								'concepts/transfers/transfer-policies',
							    'concepts/transfers/transfer-to-object'],
					},
					'concepts/versioning',
				],
			},
			{
				type: 'category',
				label: 'Move Overview',
				link: {
					type: 'doc',
					id: 'concepts/aiy-move-concepts',
				},
				items: [
					{
						type: 'category',
						label: 'Packages',
						link: {
							type: 'doc',
							id: 'concepts/aiy-move-concepts/packages',
						},
						items: [
							'concepts/aiy-move-concepts/packages/upgrade',
							'concepts/aiy-move-concepts/packages/custom-policies',
							'concepts/aiy-move-concepts/packages/automated-address-management',
						],
					},
					{
						type: 'category',
						label: 'Dynamic Fields',
						link: {
							type: 'doc',
							id: 'concepts/dynamic-fields',
						},
						items: ['concepts/dynamic-fields/tables-bags'],
					},
					'concepts/aiy-move-concepts/derived-objects',
					'concepts/aiy-move-concepts/conventions'
					
				],
			},
			{
				type: 'category',
				label: 'Transactions',
				link: {
					type: 'doc',
					id: 'concepts/transactions',
				},
				items: [
					'concepts/transactions/prog-txn-blocks',
					'concepts/transactions/sponsored-transactions',
					'concepts/transactions/gas-smashing',
				],
			},
			'concepts/grpc-overview',
			'concepts/gaming'
		],
	},
	{ 
		type: 'category',
		label: 'GraphQL and Indexer Framework',
		link: {
			type: 'doc',
			id: 'concepts/graphql-indexer',
		},
		items: [
			'concepts/graphql-rpc',
			'concepts/custom-indexing-framework',
			'concepts/custom-indexer/pipeline-architecture',
			'concepts/archival-store'
		]
	},
	{
		type: 'category',
		label: 'Cryptography',
		link: {
			type: 'doc',
			id: 'concepts/cryptography',
		},
		items: [
			{
				type: 'category',
				label: 'Transaction Authentication',
				link: {
					type: 'doc',
					id: 'concepts/cryptography/transaction-auth',
				},
				items: [
					'concepts/cryptography/transaction-auth/keys-addresses',
					'concepts/cryptography/transaction-auth/signatures',
					'concepts/cryptography/transaction-auth/multisig',
					'concepts/cryptography/transaction-auth/offline-signing',
					'concepts/cryptography/transaction-auth/intent-signing',
				],
			},
			'concepts/cryptography/zklogin',
			'concepts/cryptography/passkeys',
			{
				type: 'category',
				label: 'Nautilus',
				link: {
					type: 'doc',
					id: 'concepts/cryptography/nautilus',
				},
				items: [
					'concepts/cryptography/nautilus/nautilus-design',
					'concepts/cryptography/nautilus/using-nautilus',
				],
			},
			'concepts/cryptography/system/checkpoint-verification',
			/*{
				type: 'category',
				label: 'System',
				link: {
					type: 'doc',
					id: 'concepts/cryptography/system',
				},
				items: [
					'concepts/cryptography/system/validator-signatures',
					'concepts/cryptography/system/intents-for-validation',
					'concepts/cryptography/system/checkpoint-verification',
				],
			},*/
		],
	},
	{
		type: 'category',
		label: 'Aiy Architecture',
		link: {
			type: 'doc',
			id: 'concepts/aiy-architecture',
		},
		items: [
			'concepts/aiy-architecture/high-level',
			'concepts/aiy-architecture/aiy-storage',
			'concepts/aiy-architecture/aiy-security',
			'concepts/aiy-architecture/transaction-lifecycle',
			'concepts/aiy-architecture/consensus',
			'concepts/aiy-architecture/indexer-functions',
			'concepts/aiy-architecture/epochs',
			'concepts/aiy-architecture/protocol-upgrades',
			'concepts/aiy-architecture/data-management-things',
			'concepts/aiy-architecture/staking-rewards',
		],
	},
	{
		type: 'category',
		label: 'Tokenomics',
		link: {
			type: 'doc',
			id: 'concepts/tokenomics',
		},
		items: [
			'concepts/tokenomics/staking-unstaking',
			'concepts/tokenomics/aiy-bridging',
			'concepts/tokenomics/gas-pricing',
			'concepts/tokenomics/gas-in-aiy',
			'concepts/tokenomics/vesting-strategies'
		],
	},
	'concepts/research-papers',
];
export default concepts;
