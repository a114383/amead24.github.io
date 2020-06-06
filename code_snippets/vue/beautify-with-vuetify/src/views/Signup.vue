<template>
  <v-container>
    <v-row>
      <v-col>
        <h1>Signup</h1>
        <v-form ref="signUpForm" v-model="formValidity">
          <v-text-field label="Email" type="email" v-model="email" :rules="emailRules"></v-text-field>
          <v-autocomplete
            label="Which browser do you use?"
            :items="browsers"
          ></v-autocomplete>
          <v-file-input label="Attach profile picture"></v-file-input>
          <v-text-field
            v-model="birthday"
            label="Birthday"
            readonly
          ></v-text-field>
          <v-date-picker v-model="birthday"></v-date-picker>
          <v-checkbox label="Agree to terms & conditions" :rules="agreeToTermsRules" required></v-checkbox>
        
          <!-- Where are these default classes documented and why use them over veutify cols?? -->
          <v-btn type="submit" color="primary" class="mr-4" :disabled="!formValidity">Submit</v-btn>
          <v-btn color="warning" @click="resetValidation" class="mr-4">Reset Validation</v-btn>
          <v-btn color="error" @click="resetForm" class="mr-4">Reset</v-btn>
        </v-form>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
export default {
  data: () => ({
    agreeToTerms: false,
    agreeToTermsRules: [
        terms => !!terms || "You must agree, or else!"
    ],
    email: "",
    emailRules: [
        email => !!email || "Email empty",
        email => email.indexOf("@") !== 0 || "Email @ missing",
        email => email.includes("@") || "Missing @",
        email => email.indexOf(".") - email.indexOf("@") > 1 || "Seems fishy",
    ],
    birthday: '',
    browsers: ['Chrome', 'Firefox', 'Safari', 'Edge', 'Brave'],
    formValidity: false
  }),
  methods: {
    resetForm() {
      this.$refs.signUpForm.reset()
    },
    resetValidation() {
      // See v-form ref
      this.$refs.signUpForm.resetValidation()
    }
  }
}
</script>
