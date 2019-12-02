<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Delete an employee record</description>
   <name>Delete Employee</name>
   <tag></tag>
   <elementGuidId>e0390fef-7010-4ab3-8325-abcc1efb044a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/delete/${employee_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>2915</defaultValue>
      <description></description>
      <id>df13e5dd-39db-48d1-afef-eca7b14be678</id>
      <masked>false</masked>
      <name>employee_id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



assertThat(response.getStatusCode()).isIn(Arrays.asList(200, 201, 202))

//assertThat(response.getResponseText()).isEqualTo(&quot;success&quot;)

//assertThat(response.getResponseText()).isEqualTo(&quot;[{&quot;success&quot;:{&quot;text&quot;:&quot;successfully! deleted Records&quot;}}]&quot;)

assertThat(response.getResponseText()).contains('successfully! deleted Records')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
