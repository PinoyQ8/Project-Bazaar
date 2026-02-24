"use client";

import React, {
  createContext,
  useContext,
  useState,
  useEffect,
  type ReactNode,
} from "react";
import { PI_NETWORK_CONFIG, BACKEND_URLS } from "@/lib/system-config";
import { api, setApiAuthToken } from "@/lib/api";
import {
  initializeGlobalPayment,
  checkIncompletePayments,
} from "@/lib/pi-payment";

export type LoginDTO = {
  id: string;
  username: string;
  credits_balance: number;
  terms_accepted: boolean;
  app_id: string;
};

export type Product = {
  id: string;
  name: string;
  description: string;
  price_in_pi: number;
  total_quantity: number;
  is_active: boolean;
  created_at: string;
};

export type ProductList = {
  products: Product[];
};

const COMMUNICATION_REQUEST_TYPE = '@pi:app:sdk:communication_information_request';
const DEFAULT_ERROR_MESSAGE = 'Failed to authenticate or login. Please refresh and try again.';

function isInIframe(): boolean {
  try {
    return window.self !== window.top;
  } catch (error) {
    // Cross-origin access may throw when in an iframe
    if (
      error instanceof DOMException &&
      (error.name === 'SecurityError' || error.code === DOMException.SECURITY_ERR || error.code === 18)
    ) {
      return true;
    }
    // Firefox may throw generic Permission denied errors
    if (error instanceof Error && /Permission denied/i.test(error.message)) {
      return true;
    }

    throw error;
  }
}

function parseJsonSafely(value: any): any {
  if (typeof value === 'string') {
    try {
      return JSON.parse(value);
    } catch (error) {
      return null;
    }
  }
  return typeof value === 'object' && value !== null ? value : null;
}

interface PiAuthContextType {
  isAuthenticated: boolean;
  authMessage: string;
  hasError: boolean;
  piAccessToken: string | null;
  userData: LoginDTO | null;
  error: string | null;
  reinitialize: () => Promise<void>;
  appId: string | null;
  products: Product[] | null;
}

const PiAuthContext = createContext<PiAuthContextType | undefined>(undefined);

// Bazaar Tech: Hard-coding the Global SDK to break the loop
const loadPiSDK = (): Promise<void> => {
  return new Promise((resolve, reject) => {
    const script = document.createElement("script");
    
    // PRIORITY FIX: Direct path to Pi Mainnet logic
    script.src = "https://sdk.minepi.com/pi-sdk.js"; 
    
    script.async = true;
    script.onload = () => {
      console.log("✅ Pi SDK script loaded successfully");
      resolve();
    };
    script.onerror = () => reject(new Error("Failed to load Pi SDK"));
    document.head.appendChild(script);
  });
};

/**
 * Requests authentication credentials from the parent window (App Studio) via postMessage.
 * Returns null if not in iframe, timeout, or missing token (non-fatal check).
 *
 * @returns {Promise<{accessToken: string, appId: string}|null>} Resolves with credentials or null
 */
function requestParentCredentials(): Promise<{ accessToken: string; appId: string | null } | null> {
  // Early return if not in an iframe
  if (!isInIframe()) {
    return Promise.resolve(null);
  }

  const requestId = `${Date.now()}-${Math.random().toString(16).slice(2)}`;
  const timeoutMs = 1500;

  return new Promise((resolve) => {
    let timeoutId: ReturnType<typeof setTimeout> | null = null;

    // Cleanup function to remove listener and clear timeout
    const cleanup = (listener: (event: MessageEvent) => void) => {
      window.removeEventListener('message', listener);
      if (timeoutId !== null) {
        clearTimeout(timeoutId);
      }
    };

    const messageListener = (event: MessageEvent) => {
      // Security: only accept messages from parent window
      if (event.source !== window.parent) {
        return;
      }

      // Validate message type and request ID match
      const data = parseJsonSafely(event.data);
      if (!data || data.type !== COMMUNICATION_REQUEST_TYPE || data.id !== requestId) {
        return;
      }

      cleanup(messageListener);

      // Extract credentials from response payload
      const payload = typeof data.payload === 'object' && data.payload !== null ? data.payload : {};
      const accessToken = typeof payload.accessToken === 'string' ? payload.accessToken : null;
      const appId = typeof payload.appId === 'string' ? payload.appId : null;

      // Return credentials or null if missing token
      resolve(accessToken ? { accessToken, appId } : null);
    };

    // Set timeout handler (resolve with null on timeout)
    timeoutId = setTimeout(() => {
      cleanup(messageListener);
      resolve(null);
    }, timeoutMs);

    // Register listener before sending request to avoid race condition
    window.addEventListener('message', messageListener);

    // Send request to parent window to get credentials
    window.parent.postMessage(
      JSON.stringify({
        type: COMMUNICATION_REQUEST_TYPE,
        id: requestId
      }),
      '*'
    );
  });
}

export function PiAuthProvider({ children }: { children: ReactNode }) {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [authMessage, setAuthMessage] = useState("Initializing Pi Network...");
  const [hasError, setHasError] = useState(false);
  const [piAccessToken, setPiAccessToken] = useState<string | null>(null);
  const [userData, setUserData] = useState<LoginDTO | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [appId, setAppId] = useState<string | null>(null);
  const [products, setProducts] = useState<Product[] | null>(null);

  const fetchProducts = async (currentAppId: string): Promise<void> => {
    try {
      const { data } = await api.get<ProductList>(
        BACKEND_URLS.GET_PRODUCTS(currentAppId)
      );
      setProducts(data?.products ?? []);
    } catch (e) {
      console.error("Failed to load products:", e);
    }
  };

  const loginToBackend = async (accessToken: string, appId: string | null): Promise<LoginDTO> => {
    let endpoint: string;
    let payload: { pi_auth_token: string; app_id?: string };

    if (appId) {
      endpoint = BACKEND_URLS.LOGIN_PREVIEW;
      payload = { pi_auth_token: accessToken, app_id: appId };
    } else {
      endpoint = BACKEND_URLS.LOGIN;
      payload = { pi_auth_token: accessToken };
    }

    const loginRes = await api.post<LoginDTO>(endpoint, payload);
    return loginRes.data;
  };

  // Bazaar Tech: Mock Bypass to break the rotation loop
const authenticateAndLogin = async (accessToken: string, appId: string | null): Promise<void> => {
  setAuthMessage("Bypassing Backend (Mock Mode)...");

  // HARD-CODED: Creating a "Sovereign Identity" without needing the server
  const mockUserData: LoginDTO = {
    id: "bazaar-founder-static",
    username: "Bazaar_Founder", // Manifests your title on the S23
    credits_balance: 314.159,   // Static Pi balance for testing
    terms_accepted: true,
    app_id: appId || "bulacan-pi-node"
  };

  // Wait 1 second to simulate the handshake, then manifest
  await new Promise(resolve => setTimeout(resolve, 1000));

  setPiAccessToken(accessToken);
  setApiAuthToken(accessToken);
  setUserData(mockUserData);
  setAppId(mockUserData.app_id);
  
  console.log("✅ MESH-SCAN: Mock Login Successful. Dashboard manifesting.");
};

 const getErrorMessage = (error: unknown): string => {
    if (!(error instanceof Error))
      return "An unexpected error occurred. Please try again.";

    const errorMessage = error.message;
    if (errorMessage.includes("SDK failed to load"))
      return "Failed to load Pi Network SDK.";
    if (errorMessage.includes("authenticate"))
      return "Pi Network authentication failed.";
    if (errorMessage.includes("login"))
      return "Failed to connect to backend server.";

    return `Authentication error: ${errorMessage}`;
  };

  // Bazaar Tech: The Unified Sovereign Handshake (Only Declare Once)
  const initializePiAndAuthenticate = async (): Promise<void> => {
    setError(null);
    setHasError(false);
    try {
      // --- SOVEREIGN BYPASS START ---
      setAuthMessage("Sovereign MESH Detected. Manifesting...");
      
      const mockToken = "mock_sovereign_token_v23";
      await authenticateAndLogin(mockToken, "bulacan-pi-node");

      setIsAuthenticated(true);
      setHasError(false);
      initializeGlobalPayment();
      
      console.log("✅ MESH-SCAN: Dashboard Fluid. Rotation Terminated.");
      return; 
      // --- SOVEREIGN BYPASS END ---
    } catch (err) {
      console.error("❌ Pi Network initialization failed:", err);
      setHasError(true);
      const errorMessage = getErrorMessage(err);
      setAuthMessage(errorMessage);
      setError(errorMessage);
    }
  };

  useEffect(() => {
    initializePiAndAuthenticate();
  }, []);

  useEffect(() => {
    if (!appId) return;
    fetchProducts(appId);
  }, [appId]);

  const value: PiAuthContextType = {
    isAuthenticated,
    authMessage,
    hasError,
    piAccessToken,
    userData,
    error,
    reinitialize: initializePiAndAuthenticate,
    appId,
    products,
  };

  return (
    <PiAuthContext.Provider value={value}>{children}</PiAuthContext.Provider>
  );
} // <--- Final Seal for PiAuthProvider

export function usePiAuth() {
  const context = useContext(PiAuthContext);
  if (context === undefined) {
    throw new Error("usePiAuth must be used within a PiAuthProvider");
  }
  return context;
}
  function authenticateViaPiSdk() {
    throw new Error("Function not implemented.");
  }
