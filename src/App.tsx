import React, { useState } from "react";
// import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
"use client"
import { Box, Button, FieldLabel, FieldRoot, Group, Input, Checkbox, Code, Stack, Text } from "@chakra-ui/react"
import {
  StepsCompletedContent,
  StepsContent,
  StepsItem,
  StepsList,
  StepsNextTrigger,
  StepsPrevTrigger,
  StepsRoot,
} from "./components/ui/steps"


function App() {
  const [mail, setMail] = useState("");
  const [phone, setPhone] = useState("");
  const [userName, setUserName] = useState("");
  const [userFname, setFname] = useState("");
  const [userLname, setLname] = useState("");
  const [birthDate, setBirth] = useState("");
  const [isChecked1, setIsChecked1] = useState(false);
  const [isChecked2, setIsChecked2] = useState(false);
  const [isChecked3, setIsChecked3] = useState(false);
  const [result, setResult] = useState("")

  const [currentStep, setCurrentStep] = useState(0);
  const totalSteps = 3;

  const handleChange1 = (checked: boolean) => {
    setIsChecked1(checked)
  }
  const handleChange2 = (checked: boolean) => {
    setIsChecked2(checked)
  }
  const handleChange3 = (checked: boolean) => {
    setIsChecked3(checked)
  }
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    // setGreetMsg(await invoke("greet", { name }));
    // await invoke("data", {mail, phone, userName, userFname, userLname, birthDate});
    setResult(await invoke("data", {mail, phone, userName, userFname, userLname, birthDate}))
  }

  const handleNext = async () => {
    if (currentStep < totalSteps - 1) {
      setCurrentStep(currentStep + 1);
    } else {
      await greet();
    }
  };

  const handlePrev = () => {
    if (currentStep > 0) {
      setCurrentStep(currentStep - 1);
    }
  };

  return (
    <main className="container">
    <div>
      <StepsRoot defaultValue={0} count={totalSteps}>
        <StepsList>
          <StepsItem index={0} title="Step 1" description="This step" />
          <StepsItem index={1} title="Step 2" description="That step" />
          <StepsItem index={2} title="Step 3" description="Final step" />
        </StepsList>

        <StepsContent index={0}>
          <Box display={'flex'} flexDirection={"column"} gap={3} mx={60} my={10}>
            <FieldRoot>
              <FieldLabel>Email: </FieldLabel>
                <Input m={0} value={mail} type={"text"} variant={"subline"} name={'mail'} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setMail(e.target.value)} required />
            </FieldRoot>
            <FieldRoot>
              <FieldLabel>Phone number: </FieldLabel>
                <Input m={0} value={phone} type={"text"} variant={"subline"} name={'phone'} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPhone(e.target.value)} required />
            </FieldRoot>
            <FieldRoot>
              <FieldLabel>Username: </FieldLabel>
                <Input m={0} value={userName} type={"text"} variant={"subline"} name={'userName'} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setUserName(e.target.value)} required />
            </FieldRoot>
          </Box>
        </StepsContent>
        <StepsContent index={1}>
          <Box display={'flex'} flexDirection={"column"} gap={3} mx={60} my={10}>
            <FieldRoot>
              <FieldLabel>First Name: </FieldLabel>
                <Input m={0} value={userFname} type={"text"} variant={"subline"} name={'fisrtName'} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setFname(e.target.value)} required />
            </FieldRoot>
            <FieldRoot>
              <FieldLabel>Last Name: </FieldLabel>
                <Input m={0} value={userLname} type={"text"} variant={"subline"} name={'lastName'} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setLname(e.target.value)} required />
            </FieldRoot>
            <FieldRoot>
              <FieldLabel>Birthday: </FieldLabel>
                <Input m={0} value={birthDate} type={"text"} variant={"subline"} name={'Birthday'} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setBirth(e.target.value)} required />
            </FieldRoot>
          </Box>
        </StepsContent>
        <StepsContent index={2}>
          <Box display={'flex'} flexDirection={"column"} gap={3} mx={60} my={10} alignItems="center" justifyContent="center">
            <Stack align="flex">
              <Checkbox.Root
                isChecked={isChecked1}
                onChange={(e:React.ChangeEvent<HTMLInputElement>) => handleChange1(e.target.checked)}>
                 <Checkbox.HiddenInput />
                <Checkbox.Control />
                Accept terms and conditions
              </Checkbox.Root>
              <Code>Checked: {JSON.stringify(isChecked1, null, 2)}</Code>
              <Checkbox.Root
                isChecked={isChecked2}
                onChange={(e:React.ChangeEvent<HTMLInputElement>) => handleChange2(e.target.checked)}>
                 <Checkbox.HiddenInput />
                <Checkbox.Control />
                Agree to the terms and conditions
              </Checkbox.Root>
              <Code>Checked: {JSON.stringify(isChecked2, null, 2)}</Code>
              <Checkbox.Root
                isChecked={isChecked3}
                onChange={(e:React.ChangeEvent<HTMLInputElement>) => handleChange3(e.target.checked)}>
                 <Checkbox.HiddenInput />
                <Checkbox.Control />
                Consent to the terms and policies
              </Checkbox.Root>
              <Code>Checked: {JSON.stringify(isChecked3, null, 2)}</Code>
            </Stack>
          </Box>
        </StepsContent>
        <StepsCompletedContent>All steps are complete!</StepsCompletedContent>

        <Group style={{ display: 'flex', justifyContent: 'center', gap: '10px' }}>
          <StepsPrevTrigger asChild>
            <Button variant="outline" size="sm" onClick={handlePrev}>
              Prev
            </Button>
          </StepsPrevTrigger>
          <StepsNextTrigger asChild>
            <Button variant="outline" size="sm" onClick={handleNext}>
            {currentStep === totalSteps - 1 ? "Submit" : "Next"}
            </Button>
          </StepsNextTrigger>
        </Group>
      </StepsRoot>
      <Text>{result}</Text>

    </div>
    </main>
  );
}


export default App;


{/* <main className="container">


<form
  onSubmit={(e) => {
    e.preventDefault();
    greet();
  }}
>
  <input
    onChange={(e) => setMail(e.currentTarget.value)}
    placeholder="Enter a mail..."
  />
  <br></br>
  <br></br>

  <input
    onChange={(e) => setPhone(e.currentTarget.value)}
    placeholder="Enter a phone number..."
  />
          <br></br>
          <br></br>
  <button type="submit">Greet</button>
</form>
<p>{dataMsg}</p>
</main> */}