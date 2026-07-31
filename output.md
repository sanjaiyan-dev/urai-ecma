# Project Technical Context & LLM Prompt

# Project Title: sample-ecma-app 

## Project Description

Sample React & Express app for urai-ecma testing

Dependencies used in this project:    - **express** : `^4.18.2`
   - **react** : `^18.2.0`
   - **react-dom** : `^18.2.0`

#### Project Version: 1.2.0 


---

# Project Title: sample-ecma-app 

## Project Description

Sample React & Express app for urai-ecma testing

Dependencies used in this project:    - **express** : `^4.18.2`
   - **react** : `^18.2.0`
   - **react-dom** : `^18.2.0`

#### Project Version: 1.2.0 

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
            └── math.ts
```

### Module Dependency Graph

```mermaid
graph TD;
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
    res.json({
        users: [
            'Alice',
            'Bob'
        ]
    });
});
app.post('/api/auth/login', (req, res)=>{
    res.json({
        token: 'jwt-token-sample'
    });
});
app.delete('/api/users/:id', (req, res)=>{
    res.status(204).send();
});
app.listen(3000, ()=>{
    console.log('Server running on port 3000');
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
    const [profile, setProfile] = useState<any>(null);
    const [loading, setLoading] = useState<boolean>(true);
    useEffect(()=>{
        setLoading(true);
        fetch(`/api/users/${userId}`).then((res)=>res.json()).then((data)=>{
            setProfile(data);
            setLoading(false);
        });
    }, [
        userId
    ]);
    const handleRefresh = ()=>{
        setLoading(true);
    };
    if (loading) {
        return <div className="/* UI: Centered content in a dark gray card with padding and shadow. */">Loading profile...</div>;
    }
    return (<div className={`p-6 rounded-2xl ${isActive ? 'bg-indigo-900 border-indigo-500' : 'bg-gray-800 border-gray-600'} flex flex-col gap-4 shadow-2xl`}>
            <h2 className="/* UI: Large bold white text with tight character spacing */">{profile?.name}</h2>
            <p className="/* UI: Small, subdued gray text font */">User ID: {userId}</p>
            <button onClick={handleRefresh} className="/* UI: Pill-shaped blue button with white text and smooth hover effect. */">
                Refresh Profile
            </button>
        </div>);
};

```

### File: `sample_project/src/utils/math.ts`

**Function Summaries**:
- Line 7: `calculateDiscountPrice` -> *Calculates the discounted price of an item by applying a given percentage rate to a non-negative original cost.*

```typescript
export function calculateDiscountPrice(originalPrice: number, discountPercentage: number): number {
    if (originalPrice <= 0) {
        return 0;
    }
    if (discountPercentage <= 0) {
        return originalPrice;
    }
    const savings = (originalPrice * discountPercentage) / 100;
    const finalPrice = originalPrice - savings;
    return Math.max(0, finalPrice);
}

```


