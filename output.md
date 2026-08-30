# Project Title: sample-ecma-app 

## Project Description

Sample React & Express app for urai-ecma testing

Dependencies used in this project: 
   - **express** : `^4.18.2`
   - **react** : `^18.2.0`
   - **react-dom** : `^18.2.0`

Dev dependencies used in this project: 
   - **typescript** : `^5.0.0`

#### Project Version: 1.2.0 


---

## Project File Structure & PEG Graph

```
📁 test
├── index.ts
├── output.md
├── package.json
└── sample_project/
    ├── package.json
    └── src/
        ├── components/
        │   └── UserProfile.tsx
        ├── server.ts
        └── utils/
            ├── class.ts
            └── math.ts
```

### Module Dependency Graph

```mermaid
graph LR;
```

---

## Backend API Route Table

| Framework | Method | Path | Handler | File Location |
| :--- | :--- | :--- | :--- | :--- |
| **Express** | `GET` | `/api/users` | `anonymous_handler` | `sample_project/src/server.ts:5` |
| **Express** | `POST` | `/api/auth/login` | `anonymous_handler` | `sample_project/src/server.ts:9` |
| **Express** | `DELETE` | `/api/users/:id` | `anonymous_handler` | `sample_project/src/server.ts:13` |

---

## React Component Architecture & Explanations

### React Component Breakdown: `<UserProfile>` 

- **Props**:
  - `userId` (type: `any`)
  - `isActive` (type: `any`) [optional]
- **State Management**:
  - Manages state `profile` via setter `setProfile`.
  - Manages state `loading` via setter `setLoading`.
- **Hooks**: Uses `useState, useEffect` (Total Side-Effects: 1).
- **Rendered JSX Tree**: `<div>, <h2>, <p>, <button>` 

---

## AST-Pruned Source Code Repository

> Note: Tailwind classNames and static styles have been pruned according to mode to maximize token efficiency.

### File: `index.ts`

```typescript
const san = [
    2,
    3,
    4,
    5,
    4
];
let wow = san.map((n)=>n ** n).filter((e)=>e !== e / 2);
console.log(wow);
export { wow };

```

### File: `sample_project/src/server.ts`

```typescript
import express from 'express';
const app = express();
app.get('/api/users', (req, res)=>{
    '/* "Retrieves a static list of users by responding to GET requests at the /api/users endpoint." */';
});
app.post('/api/auth/login', (req, res)=>{
    '/* "Responds to a POST request at the login API endpoint by returning a pre-generated JWT token in the JSON response body." */';
});
app.delete('/api/users/:id', (req, res)=>{
    '/* "Deletes a specific user identified by the ID parameter in the request." */';
});
app.listen(3000, ()=>{
    '/* "Starts the application server and logs a confirmation message upon successful binding to port 3000." */';
});

```

### File: `sample_project/src/components/UserProfile.tsx`

```typescript
import React, { useState, useEffect } from 'react';
interface UserProfileProps {
    userId: string;
    isActive?: boolean;
}
export const UserProfile: React.FC<UserProfileProps> = ({ userId, isActive = true })=>{
    useEffect(()=>{
        '/* "Fetches and updates the component\'s state with user profile data whenever the user ID changes, managing loading status during the API request." */';
    }, [
        userId
    ]);
    const handleRefresh = ()=>{
        '/* "Toggles the loading state, conditionally setting it to false if it was already true." */';
    };
    return (<div className={`p-6 rounded-2xl ${isActive ? 'bg-indigo-900 border-indigo-500' : 'bg-gray-800 border-gray-600'} flex flex-col gap-4 shadow-2xl`}>
            <h2>{profile?.name}</h2>
            <p>User ID: {userId}</p>
            <button onClick={handleRefresh}>
                Refresh Profile
            </button>
        </div>);
    '/* "Fetches and displays a user\'s profile data from an API, providing state management for loading status and offering a manual refresh option." */';
};

```

### File: `sample_project/src/utils/math.ts`

```typescript
export function calculateDiscountPrice(originalPrice: number, discountPercentage: number): number {
    '/* "Calculates the discounted selling price by subtracting the calculated percentage-based savings from the original price, ensuring the result is not negative." */';
}
export function jsdoctest(originalPrice: number, discountPercentage: number): number {
    '/* "Wow thsi jsdoc discriptio | Params: param ({number}) - originalPrice; param ({number}) - discountPercentage | Returns: {number}" */';
}
function testlol() {
    const lolololo = ()=>{
        '/* "The function checks if the current date object is greater than itself, which is an impossible condition, and displays an alert if it somehow were true." */';
    };
    '/* "Wow thsi jsdoc discriptio | Params: param ({number}) - originalPrice; param ({number}) - discountPercentage | Returns: {number}" */';
}

```

### File: `sample_project/src/utils/class.ts`

```typescript
export type TaskResult<T = unknown> = {
    success: true;
    data: T;
    durationMs: number;
} | {
    success: false;
    error: Error;
    durationMs: number;
};
export type PipelineEvent<TContext> = {
    type: 'TASK_STARTED';
    taskId: string;
    timestamp: number;
} | {
    type: 'TASK_COMPLETED';
    taskId: string;
    durationMs: number;
    result: unknown;
} | {
    type: 'TASK_FAILED';
    taskId: string;
    error: Error;
    willRetry: boolean;
} | {
    type: 'TASK_ROLLBACK';
    taskId: string;
    error: Error;
} | {
    type: 'PIPELINE_SUCCESS';
    context: TContext;
    totalDurationMs: number;
} | {
    type: 'PIPELINE_FAILED';
    error: Error;
    context: TContext;
};
export interface RetryPolicy {
    maxAttempts: number;
    initialDelayMs: number;
    backoffFactor: number;
    maxDelayMs: number;
}
export interface TaskDefinition<TContext extends Record<string, unknown>, TOutput = unknown> {
    id: string;
    dependencies?: string[];
    retryPolicy?: Partial<RetryPolicy>;
    timeoutMs?: number;
    execute: (context: Readonly<TContext>, signal: AbortSignal) => Promise<TOutput>;
    rollback?: (context: Readonly<TContext>, error: Error) => Promise<void>;
    reduce?: (context: TContext, output: TOutput) => TContext;
}
export interface PipelineOptions {
    concurrencyLimit?: number;
    abortSignal?: AbortSignal;
}
export class TaskPipelineEngine<TContext extends Record<string, unknown>> {
    private readonly tasks = new Map<string, TaskDefinition<TContext, any>>();
    private readonly defaultRetryPolicy: RetryPolicy = {
        maxAttempts: 3,
        initialDelayMs: 200,
        backoffFactor: 2,
        maxDelayMs: 5000
    };
    public register<TOutput>(task: TaskDefinition<TContext, TOutput>): this {
        '/* "Register a step in the pipeline. Supports chaining." */';
    }
    public compileExecutionPlan(): string[][] {
        '/* "Validates graph structure and detects cycles using Kahn\'s Algorithm." */';
    }
    private async executeWithRetry<TOutput>(task: TaskDefinition<TContext, TOutput>, context: Readonly<TContext>, parentSignal?: AbortSignal, onRetry?: (attempt: number, error: Error) => void): Promise<TOutput> {
        '/* "Executes a task with timeout, exponential backoff, and full jitter." */';
    }
    public async *streamExecution(initialContext: TContext, options: PipelineOptions = {}): AsyncIterableIterator<PipelineEvent<TContext>> {
        '/* "Executes the entire DAG, streaming step-by-step events asynchronously." */';
    }
    public async execute(initialContext: TContext, options?: PipelineOptions): Promise<TContext> {
        '/* "Helper to execute to completion and return final context." */';
    }
    private linkAbortSignals(parent: AbortSignal, child: AbortSignal): AbortSignal {
        const onAbort = ()=>controller.abort();
        '/* "--- Utility Functions ---" */';
    }
    private chunkArray<T>(array: T[], size: number): T[][] {
        '/* "Splits a given input array into smaller arrays of a specified maximum size." */';
    }
}

```


